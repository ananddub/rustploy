use crate::utils::docker::{
    core::{ArgBuilder, Cpu, Memory, Mount, Platform, Port},
    query::filter::ContainerFilter,
    ContainerSummary, DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
};
use std::fmt;
use tokio::sync::mpsc;

// ── Handle ───────────────────────────────────────────────────────────────────

pub struct ContainerHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> ContainerHandle<'a> {
    pub fn ps(&self)                                   -> ContainerQuery<'_>  { ContainerQuery::new(self.0) }
    pub fn create(&self, image: impl Into<String>)     -> ContainerCreate<'_> { ContainerCreate::new(self.0, image) }
    pub fn exec(&self, id: impl Into<String>)          -> ExecBuilder<'_>     { ExecBuilder::new(self.0, id) }
    pub fn logs(&self, id: impl Into<String>)          -> LogsBuilder<'_>     { LogsBuilder::new(self.0, id) }
    pub fn stats(&self, id: impl Into<String>)         -> StatsBuilder<'_>    { StatsBuilder::new(self.0, id) }
    pub fn prune(&self)                                -> ContainerPrune<'_>  { ContainerPrune::new(self.0) }

    pub fn start(&self, id: impl Into<String>)         -> ContainerStartBuilder<'_>   { ContainerStartBuilder::new(self.0, id) }
    pub fn stop(&self, id: impl Into<String>)          -> ContainerStopBuilder<'_>    { ContainerStopBuilder::new(self.0, id) }
    pub fn restart(&self, id: impl Into<String>)       -> ContainerRestartBuilder<'_> { ContainerRestartBuilder::new(self.0, id) }
    pub fn pause(&self, id: impl Into<String>)         -> ContainerPauseBuilder<'_>   { ContainerPauseBuilder::new(self.0, id) }
    pub fn unpause(&self, id: impl Into<String>)       -> ContainerUnpauseBuilder<'_> { ContainerUnpauseBuilder::new(self.0, id) }
    pub fn kill(&self, id: impl Into<String>)          -> ContainerKillBuilder<'_>    { ContainerKillBuilder::new(self.0, id) }
    pub fn rm(&self, id: impl Into<String>)            -> ContainerRmBuilder<'_>      { ContainerRmBuilder::new(self.0, id) }
    pub fn rename(&self, id: impl Into<String>, name: impl Into<String>) -> ContainerRenameBuilder<'_> { ContainerRenameBuilder::new(self.0, id, name) }
    pub fn update(&self, id: impl Into<String>)        -> ContainerUpdateBuilder<'_>  { ContainerUpdateBuilder::new(self.0, id) }
    pub fn wait(&self, id: impl Into<String>)          -> ContainerWaitBuilder<'_>    { ContainerWaitBuilder::new(self.0, id) }
    pub fn port(&self, id: impl Into<String>)          -> ContainerPortBuilder<'_>    { ContainerPortBuilder::new(self.0, id) }
    pub fn top(&self, id: impl Into<String>)           -> ContainerTopBuilder<'_>     { ContainerTopBuilder::new(self.0, id) }
    pub async fn inspect(&self, id: impl AsRef<str>)   -> DockerResult<serde_json::Value> {
        let out = self.0.run(["container", "inspect", id.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

// ── ContainerQuery ────────────────────────────────────────────────────────────

pub struct ContainerQuery<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ContainerQuery<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["container", "ls", "--format", "{{json .}}"]) }
    }
    pub fn all(mut self) -> Self { self.args.flag("--all"); self }
    pub fn filter(mut self, f: ContainerFilter) -> Self { self.args.filter(f); self }
    pub fn filters(mut self, fs: impl IntoIterator<Item = ContainerFilter>) -> Self {
        for f in fs { self.args.filter(f); } self
    }

    /// Dry-run: print the docker command without executing.
    pub fn print(&self) -> String { self.args.preview() }

    pub async fn list(self) -> DockerResult<Vec<ContainerSummary>> {
        self.cli.execute_json_lines(&self.args).await
    }
    pub async fn count(self) -> DockerResult<usize> { Ok(self.list().await?.len()) }
    pub async fn exists(self) -> DockerResult<bool>  { Ok(!self.list().await?.is_empty()) }
}
crate::impl_builder_opts!(ContainerQuery);

// ── RestartPolicy ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartPolicy { No, Always, OnFailure(u32), UnlessStopped }

impl fmt::Display for RestartPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::No            => write!(f, "no"),
            Self::Always        => write!(f, "always"),
            Self::OnFailure(n)  => write!(f, "on-failure:{n}"),
            Self::UnlessStopped => write!(f, "unless-stopped"),
        }
    }
}

// ── ContainerCreate ───────────────────────────────────────────────────────────

pub struct ContainerCreate<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,   // all options
    image: String,
    command: Vec<String>,
}

impl<'a> ContainerCreate<'a> {
    fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::default(), image: image.into(), command: vec![] }
    }

    pub fn name(mut self, v: impl Into<String>)      -> Self { self.args.pair("--name", v.into()); self }
    pub fn hostname(mut self, v: impl Into<String>)  -> Self { self.args.pair("--hostname", v.into()); self }
    pub fn workdir(mut self, v: impl Into<String>)   -> Self { self.args.pair("--workdir", v.into()); self }
    pub fn user(mut self, v: impl Into<String>)      -> Self { self.args.pair("--user", v.into()); self }
    pub fn network(mut self, v: impl Into<String>)   -> Self { self.args.pair("--network", v.into()); self }

    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.env_var(k, v); self }
    pub fn envs(mut self, iter: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>) -> Self {
        for (k, v) in iter { self.args.env_var(k, v); } self
    }

    pub fn label(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.label(k, v); self }
    pub fn labels(mut self, iter: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>) -> Self {
        for (k, v) in iter { self.args.label(k, v); } self
    }

    pub fn publish(mut self, port: Port) -> Self {
        self.args.pair("--publish", port.to_string()); self
    }

    pub fn mount(mut self, m: Mount) -> Self {
        self.args.pair(m.flag_name(), m.to_string()); self
    }

    pub fn memory(mut self, m: Memory)      -> Self { self.args.pair("--memory", m.to_string()); self }
    pub fn cpus(mut self, c: Cpu)            -> Self { self.args.pair("--cpus", c.to_string()); self }
    pub fn restart(mut self, p: RestartPolicy) -> Self { self.args.pair("--restart", p.to_string()); self }
    pub fn platform(mut self, p: Platform)  -> Self { self.args.pair("--platform", p.to_string()); self }

    pub fn entrypoint(mut self, ep: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.pair("--entrypoint", ep.into_iter().map(Into::into).collect::<Vec<_>>().join(" ")); self
    }
    pub fn command(mut self, cmd: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.command = cmd.into_iter().map(Into::into).collect(); self
    }

    /// Allocate a pseudo-TTY. Pass `std::io::IsTerminal::is_terminal(&std::io::stdin())`.
    pub fn tty(mut self, enabled: bool)  -> Self { self.args.flag_if("--tty", enabled); self }
    pub fn interactive(mut self)          -> Self { self.args.flag("--interactive"); self }
    pub fn detach(mut self)               -> Self { self.args.flag("--detach"); self }
    pub fn rm(mut self)                   -> Self { self.args.flag("--rm"); self }
    pub fn privileged(mut self)           -> Self { self.args.flag("--privileged"); self }
    pub fn cap_add(mut self, cap: impl Into<String>) -> Self { self.args.pair("--cap-add", cap.into()); self }
    pub fn cap_drop(mut self, cap: impl Into<String>) -> Self { self.args.pair("--cap-drop", cap.into()); self }
    pub fn add_host(mut self, host_ip: impl Into<String>) -> Self { self.args.pair("--add-host", host_ip.into()); self }
    pub fn dns(mut self, dns: impl Into<String>) -> Self { self.args.pair("--dns", dns.into()); self }
    pub fn shm_size(mut self, size: impl Into<String>) -> Self { self.args.pair("--shm-size", size.into()); self }
    pub fn security_opt(mut self, opt: impl Into<String>) -> Self { self.args.pair("--security-opt", opt.into()); self }
    pub fn init(mut self) -> Self { self.args.flag("--init"); self }
    pub fn device(mut self, dev: impl Into<String>) -> Self { self.args.pair("--device", dev.into()); self }
    pub fn network_alias(mut self, alias: impl Into<String>) -> Self { self.args.pair("--network-alias", alias.into()); self }
    /// Pass any raw flag not covered by the builder.
    pub fn arg(mut self, v: impl Into<String>) -> Self { self.args.push(v.into()); self }

    fn finalize(mut self, subcmd: &str) -> (ArgBuilder, &'a DockerCli) {
        let mut full = ArgBuilder::cmd(&["container", subcmd]);
        full.inherit_meta(&self.args);
        full.push_all(self.args.build());
        full.push(&self.image);
        full.push_all(self.command.drain(..));
        (full, self.cli)
    }

    /// Dry-run: print the full `docker container run …` command.
    pub fn print_run(&self) -> String {
        let mut a = ArgBuilder::cmd(&["container", "run"]);
        let opts = self.args.clone().build();
        a.push_all(opts);
        a.push(&self.image);
        a.push_all(self.command.clone());
        a.preview()
    }

    /// `docker container create …` — returns the new container ID.
    pub async fn create(self) -> DockerResult<String> {
        let cli = self.cli;
        let (args, _) = self.finalize("create");
        Ok(cli.execute(&args).await?.stdout.trim().to_string())
    }

    /// `docker container run …`
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let (args, cli) = self.finalize("run");
        cli.execute(&args).await
    }

    /// `docker container run …` — streaming output.
    pub async fn run_stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        let (args, cli) = self.finalize("run");
        cli.execute_stream(&args, sender).await
    }
}
crate::impl_builder_opts!(ContainerCreate);

// ── ExecBuilder ───────────────────────────────────────────────────────────────

pub struct ExecBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }

impl<'a> ExecBuilder<'a> {
    fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self { cli, id: id.into(), args: ArgBuilder::default() }
    }
    pub fn user(mut self, v: impl Into<String>)    -> Self { self.args.pair("--user", v.into()); self }
    pub fn workdir(mut self, v: impl Into<String>) -> Self { self.args.pair("--workdir", v.into()); self }
    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.env_var(k, v); self }
    pub fn tty(mut self, enabled: bool)            -> Self { self.args.flag_if("--tty", enabled); self }
    pub fn interactive(mut self)                   -> Self { self.args.flag("--interactive"); self }
    pub fn detach(mut self)                        -> Self { self.args.flag("--detach"); self }

    pub fn print(&self, cmd: &[impl AsRef<str>]) -> String {
        let mut a = ArgBuilder::cmd(&["container", "exec"]);
        a.push_all(self.args.clone().build());
        a.push(&self.id);
        a.push_all(cmd.iter().map(|s| s.as_ref().to_string()));
        a.preview()
    }

    pub async fn run(self, cmd: impl IntoIterator<Item = impl Into<String>>) -> DockerResult<DockerOutput> {
        let mut a = ArgBuilder::cmd(&["container", "exec"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        a.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute(&a).await
    }

    pub async fn run_stream(self, cmd: impl IntoIterator<Item = impl Into<String>>, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["container", "exec"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        a.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(ExecBuilder);

// ── LogsBuilder ───────────────────────────────────────────────────────────────

pub struct LogsBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }

impl<'a> LogsBuilder<'a> {
    fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self { cli, id: id.into(), args: ArgBuilder::default() }
    }
    pub fn follow(mut self)                        -> Self { self.args.flag("--follow"); self }
    pub fn timestamps(mut self)                    -> Self { self.args.flag("--timestamps"); self }
    pub fn tail(mut self, n: usize)                -> Self { self.args.pair("--tail", n.to_string()); self }
    pub fn since(mut self, v: impl Into<String>)   -> Self { self.args.pair("--since", v.into()); self }
    pub fn until(mut self, v: impl Into<String>)   -> Self { self.args.pair("--until", v.into()); self }
    pub fn stdout(mut self)                        -> Self { self.args.flag("--stdout"); self }
    pub fn stderr(mut self)                        -> Self { self.args.flag("--stderr"); self }

    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["container", "logs"]);
        a.push_all(self.args.clone().build());
        a.push(&self.id);
        a.preview()
    }

    pub async fn output(self) -> DockerResult<String> {
        let mut a = ArgBuilder::cmd(&["container", "logs"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        let out = self.cli.execute(&a).await?;
        Ok(format!("{}{}", out.stdout, out.stderr))
    }

    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["container", "logs"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(LogsBuilder);

// ── StatsBuilder ──────────────────────────────────────────────────────────────

pub struct StatsBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }

impl<'a> StatsBuilder<'a> {
    fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self { cli, id: id.into(), args: ArgBuilder::default() }
    }
    pub fn no_stream(mut self) -> Self { self.args.flag("--no-stream"); self }

    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["container", "stats"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(StatsBuilder);

// ── ContainerPrune ────────────────────────────────────────────────────────────

pub struct ContainerPrune<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ContainerPrune<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["container", "prune", "--force"]) }
    }
    pub fn filter(mut self, f: ContainerFilter) -> Self { self.args.filter(f); self }
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ContainerPrune);

// ── Generated Simple Builders ───────────────────────────────────────────────

pub struct ContainerStartBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerStartBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "start"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerStartBuilder);

pub struct ContainerStopBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerStopBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "stop"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerStopBuilder);

pub struct ContainerRestartBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerRestartBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "restart"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerRestartBuilder);

pub struct ContainerPauseBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerPauseBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "pause"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerPauseBuilder);

pub struct ContainerUnpauseBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerUnpauseBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "unpause"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerUnpauseBuilder);

pub struct ContainerKillBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerKillBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "kill"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerKillBuilder);

pub struct ContainerWaitBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerWaitBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "wait"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerWaitBuilder);

pub struct ContainerPortBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerPortBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "port"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerPortBuilder);

pub struct ContainerTopBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerTopBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "top"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerTopBuilder);

pub struct ContainerRmBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerRmBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "rm"]) } }
    pub fn force(mut self) -> Self { self.args.flag("--force"); self }
    pub fn volumes(mut self) -> Self { self.args.flag("--volumes"); self }
    pub fn link(mut self) -> Self { self.args.flag("--link"); self }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerRmBuilder);

pub struct ContainerRenameBuilder<'a> { cli: &'a DockerCli, id: String, name: String, args: ArgBuilder }
impl<'a> ContainerRenameBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>, name: impl Into<String>) -> Self { Self { cli, id: id.into(), name: name.into(), args: ArgBuilder::cmd(&["container", "rename"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); a.push(&self.name); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerRenameBuilder);

pub struct ContainerUpdateBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerUpdateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self { Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "update"]) } }
    pub fn memory(mut self, m: crate::utils::docker::core::Memory) -> Self { self.args.pair("--memory", m.to_string()); self }
    pub fn cpus(mut self, c: crate::utils::docker::core::Cpu) -> Self { self.args.pair("--cpus", c.to_string()); self }
    pub fn restart(mut self, p: crate::utils::docker::handles::containers::RestartPolicy) -> Self { self.args.pair("--restart", p.to_string()); self }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.id); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(ContainerUpdateBuilder);

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::docker::DockerCli;
    use crate::utils::docker::query::filter::ContainerStatus;

    fn cli() -> DockerCli { DockerCli::new_local() }

    #[test]
    fn container_create_args() {
        let temp = cli();
        let c = ContainerCreate::new(&temp, "nginx:latest")
            .name("web")
            .network("bridge")
            .publish(Port::tcp(8080, 80))
            .mount(Mount::volume("data", "/data"))
            .env("PORT", "80")
            .memory(Memory::mb(256))
            .cpus(Cpu::cores(0.5))
            .platform(Platform::LinuxAmd64)
            .restart(RestartPolicy::UnlessStopped)
            .tty(false)
            .detach()
            .privileged()
            .cap_add("NET_ADMIN")
            .add_host("host.docker.internal:host-gateway")
            .dns("8.8.8.8")
            .init();

        let preview = c.print_run();
        assert!(preview.contains("container run"));
        assert!(preview.contains("--name web"));
        assert!(preview.contains("8080:80/tcp"));
        assert!(preview.contains("256m"));
        assert!(preview.contains("0.50"));
        assert!(preview.contains("linux/amd64"));
        assert!(preview.contains("--privileged"));
        assert!(preview.contains("--cap-add NET_ADMIN"));
        assert!(preview.contains("--add-host host.docker.internal:host-gateway"));
        assert!(preview.contains("--dns 8.8.8.8"));
        assert!(preview.contains("--init"));
        assert!(!preview.contains("--tty"));
    }

    #[test]
    fn container_create_tty_enabled() {
        let temp = cli();
        let c = ContainerCreate::new(&temp, "alpine").tty(true);
        assert!(c.print_run().contains("--tty"));
    }

    #[test]
    fn restart_policy() {
        assert_eq!(RestartPolicy::OnFailure(3).to_string(), "on-failure:3");
        assert_eq!(RestartPolicy::UnlessStopped.to_string(), "unless-stopped");
    }

    #[test]
    fn container_query_print() {
        let tmp = cli();
        let q = ContainerQuery::new(&tmp).all()
            .filter(ContainerFilter::Status(ContainerStatus::Running));
        assert!(q.print().contains("--all"));
        assert!(q.print().contains("status=running"));
    }
}
