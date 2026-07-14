use super::filter::ContainerFilter;
use crate::utils::docker::{
    ContainerSummary, DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
};
use std::fmt;
use tokio::sync::mpsc;

// ── ContainerQuery ────────────────────────────────────────────────────────────

/// Fluent builder for `docker container ls`.
pub struct ContainerQuery<'a> {
    cli: &'a DockerCli,
    all: bool,
    filters: Vec<ContainerFilter>,
}

impl<'a> ContainerQuery<'a> {
    pub(super) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, all: false, filters: vec![] }
    }

    /// Include stopped containers (`--all`).
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// Add a single filter.
    pub fn filter(mut self, f: ContainerFilter) -> Self {
        self.filters.push(f);
        self
    }

    /// Add multiple filters at once.
    pub fn filters(mut self, fs: impl IntoIterator<Item = ContainerFilter>) -> Self {
        self.filters.extend(fs);
        self
    }

    fn args(&self) -> Vec<String> {
        let mut a = vec![
            "container".into(), "ls".into(),
            "--format".into(), "{{json .}}".into(),
        ];
        if self.all { a.push("--all".into()); }
        for f in &self.filters {
            a.extend(["--filter".into(), f.to_string()]);
        }
        a
    }

    /// Execute and return matching containers.
    pub async fn list(self) -> DockerResult<Vec<ContainerSummary>> {
        let args = self.args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        self.cli.json_lines(&refs).await
    }

    /// Return the count of matching containers.
    pub async fn count(self) -> DockerResult<usize> {
        Ok(self.list().await?.len())
    }

    /// Return `true` if at least one container matches.
    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}

// ── RestartPolicy ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    Always,
    /// Restart on failure up to `n` times.
    OnFailure(u32),
    UnlessStopped,
}

impl fmt::Display for RestartPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::No => write!(f, "no"),
            Self::Always => write!(f, "always"),
            Self::OnFailure(n) => write!(f, "on-failure:{n}"),
            Self::UnlessStopped => write!(f, "unless-stopped"),
        }
    }
}

// ── Protocol ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Protocol {
    #[default]
    Tcp,
    Udp,
    Sctp,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self { Self::Tcp => "tcp", Self::Udp => "udp", Self::Sctp => "sctp" })
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

struct PortBinding { host: u16, container: u16, proto: Protocol }
struct Mount { source: String, target: String, read_only: bool, kind: MountKind }
enum MountKind { Volume, Bind, Tmpfs }

// ── ContainerCreate ───────────────────────────────────────────────────────────

/// Fluent builder for `docker container create` / `docker container run`.
///
/// Terminal methods: [`.create()`], [`.run()`], [`.run_stream()`].
pub struct ContainerCreate<'a> {
    cli: &'a DockerCli,
    image: String,
    name: Option<String>,
    hostname: Option<String>,
    env: Vec<(String, String)>,
    labels: Vec<(String, String)>,
    networks: Vec<String>,
    ports: Vec<PortBinding>,
    mounts: Vec<Mount>,
    memory: Option<String>,
    cpus: Option<String>,
    restart: Option<RestartPolicy>,
    command: Vec<String>,
    entrypoint: Vec<String>,
    workdir: Option<String>,
    user: Option<String>,
    platform: Option<String>,
    /// Whether to allocate a pseudo-TTY (`-t`). Pass `true` if the caller's
    /// stdin is a terminal (e.g. `std::io::IsTerminal::is_terminal`).
    tty: bool,
    interactive: bool,
    detach: bool,
    rm: bool,
    extra: Vec<String>,
}

impl<'a> ContainerCreate<'a> {
    pub(super) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self {
            cli, image: image.into(),
            name: None, hostname: None,
            env: vec![], labels: vec![], networks: vec![],
            ports: vec![], mounts: vec![],
            memory: None, cpus: None, restart: None,
            command: vec![], entrypoint: vec![],
            workdir: None, user: None, platform: None,
            tty: false, interactive: false, detach: false, rm: false,
            extra: vec![],
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn hostname(mut self, v: impl Into<String>) -> Self { self.hostname = Some(v.into()); self }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.push((key.into(), value.into())); self
    }
    pub fn envs(mut self, iter: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>) -> Self {
        self.env.extend(iter.into_iter().map(|(k, v)| (k.into(), v.into()))); self
    }

    pub fn label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.push((key.into(), value.into())); self
    }
    pub fn labels(mut self, iter: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>) -> Self {
        self.labels.extend(iter.into_iter().map(|(k, v)| (k.into(), v.into()))); self
    }

    pub fn network(mut self, net: impl Into<String>) -> Self { self.networks.push(net.into()); self }

    /// Publish `host_port:container_port/tcp`.
    pub fn publish(mut self, host_port: u16, container_port: u16) -> Self {
        self.ports.push(PortBinding { host: host_port, container: container_port, proto: Protocol::Tcp }); self
    }
    /// Publish with an explicit protocol.
    pub fn publish_proto(mut self, host_port: u16, container_port: u16, proto: Protocol) -> Self {
        self.ports.push(PortBinding { host: host_port, container: container_port, proto }); self
    }

    /// Mount a named volume at `target`.
    pub fn volume(mut self, name: impl Into<String>, target: impl Into<String>) -> Self {
        self.mounts.push(Mount { source: name.into(), target: target.into(), read_only: false, kind: MountKind::Volume }); self
    }
    /// Bind-mount a host path at `target`.
    pub fn bind(mut self, host_path: impl Into<String>, target: impl Into<String>) -> Self {
        self.mounts.push(Mount { source: host_path.into(), target: target.into(), read_only: false, kind: MountKind::Bind }); self
    }
    /// Bind-mount read-only.
    pub fn bind_ro(mut self, host_path: impl Into<String>, target: impl Into<String>) -> Self {
        self.mounts.push(Mount { source: host_path.into(), target: target.into(), read_only: true, kind: MountKind::Bind }); self
    }
    /// tmpfs mount at `target`.
    pub fn tmpfs(mut self, target: impl Into<String>) -> Self {
        self.mounts.push(Mount { source: String::new(), target: target.into(), read_only: false, kind: MountKind::Tmpfs }); self
    }

    pub fn memory(mut self, v: impl Into<String>) -> Self { self.memory = Some(v.into()); self }
    pub fn cpus(mut self, v: impl Into<String>) -> Self { self.cpus = Some(v.into()); self }
    pub fn restart(mut self, policy: RestartPolicy) -> Self { self.restart = Some(policy); self }
    pub fn workdir(mut self, v: impl Into<String>) -> Self { self.workdir = Some(v.into()); self }
    pub fn user(mut self, v: impl Into<String>) -> Self { self.user = Some(v.into()); self }
    pub fn platform(mut self, v: impl Into<String>) -> Self { self.platform = Some(v.into()); self }

    pub fn command(mut self, cmd: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.command = cmd.into_iter().map(Into::into).collect(); self
    }
    pub fn entrypoint(mut self, ep: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.entrypoint = ep.into_iter().map(Into::into).collect(); self
    }

    /// Allocate a pseudo-TTY (`-t`).
    ///
    /// Pass the result of [`std::io::IsTerminal::is_terminal`] (or a runtime
    /// flag from the calling context) to let the caller decide whether the
    /// current process has a terminal.
    pub fn tty(mut self, enabled: bool) -> Self { self.tty = enabled; self }

    /// Keep STDIN open (`-i`).
    pub fn interactive(mut self) -> Self { self.interactive = true; self }
    /// Run in the background (`-d`).
    pub fn detach(mut self) -> Self { self.detach = true; self }
    /// Remove the container when it exits (`--rm`).
    pub fn rm(mut self) -> Self { self.rm = true; self }

    /// Pass any raw flag not yet covered by the builder.
    pub fn arg(mut self, v: impl Into<String>) -> Self { self.extra.push(v.into()); self }

    // ── arg building ─────────────────────────────────────────────────────────

    fn build_opts(&self) -> Vec<String> {
        let mut a: Vec<String> = vec![];

        if let Some(n) = &self.name     { a.extend(["--name".into(), n.clone()]); }
        if let Some(h) = &self.hostname { a.extend(["--hostname".into(), h.clone()]); }
        for (k, v) in &self.env         { a.extend(["--env".into(), format!("{k}={v}")]); }
        for (k, v) in &self.labels      { a.extend(["--label".into(), format!("{k}={v}")]); }
        for net in &self.networks        { a.extend(["--network".into(), net.clone()]); }

        for p in &self.ports {
            a.extend(["--publish".into(), format!("{}:{}/{}", p.host, p.container, p.proto)]);
        }

        for m in &self.mounts {
            match m.kind {
                MountKind::Volume => {
                    let spec = if m.read_only {
                        format!("{}:{}:ro", m.source, m.target)
                    } else {
                        format!("{}:{}", m.source, m.target)
                    };
                    a.extend(["--volume".into(), spec]);
                }
                MountKind::Bind => {
                    let spec = if m.read_only {
                        format!("type=bind,source={},target={},readonly", m.source, m.target)
                    } else {
                        format!("type=bind,source={},target={}", m.source, m.target)
                    };
                    a.extend(["--mount".into(), spec]);
                }
                MountKind::Tmpfs => {
                    a.extend(["--mount".into(), format!("type=tmpfs,target={}", m.target)]);
                }
            }
        }

        if let Some(m) = &self.memory  { a.extend(["--memory".into(), m.clone()]); }
        if let Some(c) = &self.cpus    { a.extend(["--cpus".into(), c.clone()]); }
        if let Some(r) = &self.restart { a.extend(["--restart".into(), r.to_string()]); }
        if let Some(w) = &self.workdir { a.extend(["--workdir".into(), w.clone()]); }
        if let Some(u) = &self.user    { a.extend(["--user".into(), u.clone()]); }
        if let Some(p) = &self.platform { a.extend(["--platform".into(), p.clone()]); }

        if !self.entrypoint.is_empty() {
            a.push("--entrypoint".into());
            a.push(self.entrypoint.join(" "));
        }

        if self.tty         { a.push("--tty".into()); }
        if self.interactive { a.push("--interactive".into()); }
        if self.detach      { a.push("--detach".into()); }
        if self.rm          { a.push("--rm".into()); }

        a.extend(self.extra.clone());
        a.push(self.image.clone());
        a.extend(self.command.clone());
        a
    }

    // ── terminals ────────────────────────────────────────────────────────────

    /// `docker container create …` — returns the new container ID.
    pub async fn create(self) -> DockerResult<String> {
        let cli = self.cli;
        let mut args = vec!["container".to_string(), "create".to_string()];
        args.extend(self.build_opts());
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        Ok(cli.run(refs).await?.stdout.trim().to_string())
    }

    /// `docker container run …` — returns the combined output.
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let cli = self.cli;
        let mut args = vec!["container".to_string(), "run".to_string()];
        args.extend(self.build_opts());
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        cli.run(refs).await
    }

    /// `docker container run …` — streams output events.
    pub async fn run_stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let cli = self.cli;
        let mut args = vec!["container".to_string(), "run".to_string()];
        args.extend(self.build_opts());
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        cli.run_stream(refs, sender).await
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::docker::DockerCli;

    fn fake() -> DockerCli { DockerCli::new_local() }

    #[test]
    fn restart_policy_display() {
        assert_eq!(RestartPolicy::No.to_string(), "no");
        assert_eq!(RestartPolicy::Always.to_string(), "always");
        assert_eq!(RestartPolicy::OnFailure(5).to_string(), "on-failure:5");
        assert_eq!(RestartPolicy::UnlessStopped.to_string(), "unless-stopped");
    }

    #[test]
    fn protocol_display() {
        assert_eq!(Protocol::Tcp.to_string(), "tcp");
        assert_eq!(Protocol::Udp.to_string(), "udp");
    }

    #[test]
    fn create_builds_args_correctly() {
        let cli = fake();
        let builder = ContainerCreate::new(&cli, "nginx:latest")
            .name("web")
            .env("PORT", "80")
            .label("app", "web")
            .network("bridge")
            .publish(8080, 80)
            .volume("web-data", "/data")
            .bind_ro("/etc/nginx/nginx.conf", "/etc/nginx/nginx.conf")
            .memory("256m")
            .restart(RestartPolicy::UnlessStopped)
            .tty(false)
            .interactive()
            .detach();

        let opts = builder.build_opts();
        assert!(opts.contains(&"--name".to_string()));
        assert!(opts.contains(&"web".to_string()));
        assert!(opts.contains(&"--env".to_string()));
        assert!(opts.contains(&"PORT=80".to_string()));
        assert!(opts.contains(&"8080:80/tcp".to_string()));
        assert!(opts.contains(&"--restart".to_string()));
        assert!(opts.contains(&"unless-stopped".to_string()));
        assert!(!opts.contains(&"--tty".to_string()));
        assert!(opts.contains(&"--interactive".to_string()));
        assert!(opts.contains(&"--detach".to_string()));
        // image is last option, before command
        assert_eq!(opts.last(), Some(&"nginx:latest".to_string()));
    }

    #[test]
    fn tty_enabled_when_true() {
        let cli = fake();
        let opts = ContainerCreate::new(&cli, "alpine").tty(true).build_opts();
        assert!(opts.contains(&"--tty".to_string()));
    }
}
