use crate::utils::docker::{
    core::ArgBuilder,
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent, ComposeContainer,
};
use tokio::sync::mpsc;

pub struct ComposeHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> ComposeHandle<'a> {
    pub fn up(&self)                               -> UpBuilder<'_>   { UpBuilder::new(self.0) }
    pub fn down(&self)                             -> DownBuilder<'_> { DownBuilder::new(self.0) }
    pub fn build(&self)                            -> ComposeBuildBuilder<'_> { ComposeBuildBuilder::new(self.0) }
    pub fn pull(&self)                             -> ComposePullBuilder<'_> { ComposePullBuilder::new(self.0) }
    pub fn push(&self)                             -> ComposePushBuilder<'_> { ComposePushBuilder::new(self.0) }
    pub fn exec(&self, service: impl Into<String>) -> ComposeExecBuilder<'_> { ComposeExecBuilder::new(self.0, service) }
    pub fn run(&self, service: impl Into<String>)  -> ComposeRunBuilder<'_> { ComposeRunBuilder::new(self.0, service) }
    pub fn start(&self)                            -> ComposeStartBuilder<'_> { ComposeStartBuilder::new(self.0) }
    pub fn stop(&self)                             -> ComposeStopBuilder<'_> { ComposeStopBuilder::new(self.0) }
    pub fn rm(&self)                               -> ComposeRmBuilder<'_> { ComposeRmBuilder::new(self.0) }
    pub fn config(&self)                           -> ComposeConfigBuilder<'_> { ComposeConfigBuilder::new(self.0) }
    pub fn logs(&self, service: impl Into<String>) -> ComposeLogsBuilder<'_> { ComposeLogsBuilder::new(self.0, service) }
    pub fn ps(&self)                               -> ComposePsBuilder<'_> { ComposePsBuilder::new(self.0) }
    pub fn restart(&self, services: impl IntoIterator<Item = impl Into<String>>) -> ComposeRestartBuilder<'_> {
        ComposeRestartBuilder::new(self.0, services)
    }
}



// ── UpBuilder ─────────────────────────────────────────────────────────────────

pub struct UpBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> UpBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "up"]) }
    }
    pub fn service(mut self, s: impl Into<String>)       -> Self { self.args.push(s.into()); self }
    pub fn detach(mut self)                              -> Self { self.args.flag("--detach"); self }
    pub fn build(mut self)                               -> Self { self.args.flag("--build"); self }
    pub fn no_deps(mut self)                             -> Self { self.args.flag("--no-deps"); self }
    pub fn remove_orphans(mut self)                      -> Self { self.args.flag("--remove-orphans"); self }
    pub fn force_recreate(mut self)                      -> Self { self.args.flag("--force-recreate"); self }
    pub fn no_recreate(mut self)                         -> Self { self.args.flag("--no-recreate"); self }
    pub fn no_start(mut self)                            -> Self { self.args.flag("--no-start"); self }
    pub fn quiet_pull(mut self)                          -> Self { self.args.flag("--quiet-pull"); self }
    pub fn abort_on_container_exit(mut self)             -> Self { self.args.flag("--abort-on-container-exit"); self }
    pub fn wait(mut self)                                -> Self { self.args.flag("--wait"); self }
    pub fn timeout(mut self, t: u32)                     -> Self { self.args.pair("--timeout", t.to_string()); self }
    pub fn scale(mut self, service: impl AsRef<str>, n: u32) -> Self {
        self.args.pair("--scale", format!("{}={}", service.as_ref(), n)); self
    }
    pub fn print(&self)  -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        self.cli.execute_stream(&self.args, sender).await
    }
}
crate::impl_builder_opts!(UpBuilder);
crate::impl_compose_opts!(UpBuilder);

// ── DownBuilder ───────────────────────────────────────────────────────────────

pub struct DownBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> DownBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "down"]) }
    }
    pub fn volumes(mut self)                        -> Self { self.args.flag("--volumes"); self }
    pub fn remove_orphans(mut self)                 -> Self { self.args.flag("--remove-orphans"); self }
    pub fn rmi(mut self, kind: impl Into<crate::utils::docker::RmiMode>)   -> Self {
        let k: crate::utils::docker::RmiMode = kind.into();
        self.args.pair("--rmi", k.as_str());
        self
    }
    pub fn timeout(mut self, t: u32)                -> Self { self.args.pair("--timeout", t.to_string()); self }
    pub fn print(&self)  -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(DownBuilder);
crate::impl_compose_opts!(DownBuilder);

// ── ComposeLogsBuilder ────────────────────────────────────────────────────────

pub struct ComposeLogsBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposeLogsBuilder<'a> {
    fn new(cli: &'a DockerCli, service: impl Into<String>) -> Self {
        let mut args = ArgBuilder::cmd(&["compose", "logs"]);
        args.push(service.into());
        Self { cli, args }
    }
    pub fn follow(mut self)                      -> Self { self.args.flag("--follow"); self }
    pub fn tail(mut self, n: usize)              -> Self { self.args.pair("--tail", n.to_string()); self }
    pub fn timestamps(mut self)                  -> Self { self.args.flag("--timestamps"); self }
    pub fn no_color(mut self)                    -> Self { self.args.flag("--no-color"); self }
    pub fn since(mut self, s: impl Into<String>) -> Self { self.args.pair("--since", s.into()); self }
    pub fn until(mut self, u: impl Into<String>) -> Self { self.args.pair("--until", u.into()); self }
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        self.cli.execute_stream(&self.args, sender).await
    }
}
crate::impl_builder_opts!(ComposeLogsBuilder);
crate::impl_compose_opts!(ComposeLogsBuilder);

// ── ComposePsBuilder ──────────────────────────────────────────────────────────

pub struct ComposePsBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposePsBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "ps"]) }
    }
    pub fn all(mut self)                           -> Self { self.args.flag("--all"); self }
    pub fn service(mut self, s: impl Into<String>) -> Self { self.args.push(s.into()); self }
    pub fn filter(mut self, f: impl Into<String>)  -> Self { self.args.pair("--filter", f.into()); self }
    pub fn status(mut self, s: impl Into<String>)  -> Self { self.args.pair("--status", s.into()); self }
    pub fn quiet(mut self)                         -> Self { self.args.flag("--quiet"); self }
    pub fn services(mut self)                      -> Self { self.args.flag("--services"); self }
    pub fn print(&self) -> String { self.args.preview() }
    
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
    
    pub async fn list(mut self) -> DockerResult<Vec<ComposeContainer>> {
        self.args.pair("--format", "json");
        self.cli.execute_json(&self.args).await
    }
}
crate::impl_builder_opts!(ComposePsBuilder);
crate::impl_compose_opts!(ComposePsBuilder);

// ── ComposeRestartBuilder ─────────────────────────────────────────────────────

pub struct ComposeRestartBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposeRestartBuilder<'a> {
    fn new(cli: &'a DockerCli, services: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut args = ArgBuilder::cmd(&["compose", "restart"]);
        for s in services { args.push(s.into()); }
        Self { cli, args }
    }
    pub fn timeout(mut self, t: u32) -> Self { self.args.pair("--timeout", t.to_string()); self }
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ComposeRestartBuilder);
crate::impl_compose_opts!(ComposeRestartBuilder);

// ── ComposeBuildBuilder ───────────────────────────────────────────────────────

pub struct ComposeBuildBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposeBuildBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "build"]) }
    }
    pub fn service(mut self, s: impl Into<String>) -> Self { self.args.push(s.into()); self }
    pub fn no_cache(mut self)                      -> Self { self.args.flag("--no-cache"); self }
    pub fn pull(mut self)                          -> Self { self.args.flag("--pull"); self }
    pub fn quiet(mut self)                         -> Self { self.args.flag("--quiet"); self }
    pub fn build_arg(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.pair("--build-arg", format!("{}={}", k.as_ref(), v.as_ref())); self
    }
    pub fn ssh(mut self, s: impl Into<String>)     -> Self { self.args.pair("--ssh", s.into()); self }
    pub fn progress(mut self, p: impl Into<crate::utils::docker::BuildProgress>)-> Self {
        let progress: crate::utils::docker::BuildProgress = p.into();
        self.args.pair("--progress", progress.as_str());
        self
    }
    
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        self.cli.execute_stream(&self.args, sender).await
    }
}
crate::impl_builder_opts!(ComposeBuildBuilder);
crate::impl_compose_opts!(ComposeBuildBuilder);

// ── ComposePullBuilder ────────────────────────────────────────────────────────

pub struct ComposePullBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposePullBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "pull"]) }
    }
    pub fn service(mut self, s: impl Into<String>) -> Self { self.args.push(s.into()); self }
    pub fn ignore_pull_failures(mut self)          -> Self { self.args.flag("--ignore-pull-failures"); self }
    pub fn parallel(mut self)                      -> Self { self.args.flag("--parallel"); self }
    pub fn quiet(mut self)                         -> Self { self.args.flag("--quiet"); self }
    
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        self.cli.execute_stream(&self.args, sender).await
    }
}
crate::impl_builder_opts!(ComposePullBuilder);
crate::impl_compose_opts!(ComposePullBuilder);

// ── ComposePushBuilder ────────────────────────────────────────────────────────

pub struct ComposePushBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposePushBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "push"]) }
    }
    pub fn service(mut self, s: impl Into<String>) -> Self { self.args.push(s.into()); self }
    pub fn ignore_push_failures(mut self)          -> Self { self.args.flag("--ignore-push-failures"); self }
    
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        self.cli.execute_stream(&self.args, sender).await
    }
}
crate::impl_builder_opts!(ComposePushBuilder);
crate::impl_compose_opts!(ComposePushBuilder);

// ── ComposeExecBuilder ────────────────────────────────────────────────────────

pub struct ComposeExecBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder, service: String }

impl<'a> ComposeExecBuilder<'a> {
    fn new(cli: &'a DockerCli, service: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "exec"]), service: service.into() }
    }
    pub fn detach(mut self)                                      -> Self { self.args.flag("--detach"); self }
    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.pair("--env", format!("{}={}", k.as_ref(), v.as_ref())); self }
    pub fn user(mut self, u: impl Into<String>)                  -> Self { self.args.pair("--user", u.into()); self }
    pub fn workdir(mut self, w: impl Into<String>)               -> Self { self.args.pair("--workdir", w.into()); self }
    pub fn privileged(mut self)                                  -> Self { self.args.flag("--privileged"); self }
    pub fn index(mut self, i: u32)                               -> Self { self.args.pair("--index", i.to_string()); self }
    
    pub async fn run(mut self, cmd: impl IntoIterator<Item = impl Into<String>>) -> DockerResult<DockerOutput> {
        self.args.push(&self.service);
        self.args.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute(&self.args).await
    }
    pub async fn stream(mut self, cmd: impl IntoIterator<Item = impl Into<String>>, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        self.args.push(&self.service);
        self.args.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute_stream(&self.args, sender).await
    }
}
crate::impl_builder_opts!(ComposeExecBuilder);
crate::impl_compose_opts!(ComposeExecBuilder);

// ── ComposeRunBuilder ─────────────────────────────────────────────────────────

pub struct ComposeRunBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder, service: String }

impl<'a> ComposeRunBuilder<'a> {
    fn new(cli: &'a DockerCli, service: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "run"]), service: service.into() }
    }
    pub fn build(mut self)                                       -> Self { self.args.flag("--build"); self }
    pub fn detach(mut self)                                      -> Self { self.args.flag("--detach"); self }
    pub fn entrypoint(mut self, e: impl Into<String>)            -> Self { self.args.pair("--entrypoint", e.into()); self }
    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.pair("--env", format!("{}={}", k.as_ref(), v.as_ref())); self }
    pub fn publish(mut self, p: impl Into<String>)               -> Self { self.args.pair("--publish", p.into()); self }
    pub fn rm(mut self)                                          -> Self { self.args.flag("--rm"); self }
    pub fn service_ports(mut self)                               -> Self { self.args.flag("--service-ports"); self }
    pub fn user(mut self, u: impl Into<String>)                  -> Self { self.args.pair("--user", u.into()); self }
    pub fn workdir(mut self, w: impl Into<String>)               -> Self { self.args.pair("--workdir", w.into()); self }
    pub fn volume(mut self, v: impl Into<String>)                -> Self { self.args.pair("--volume", v.into()); self }
    
    pub async fn run(mut self, cmd: impl IntoIterator<Item = impl Into<String>>) -> DockerResult<DockerOutput> {
        self.args.push(&self.service);
        self.args.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute(&self.args).await
    }
    pub async fn stream(mut self, cmd: impl IntoIterator<Item = impl Into<String>>, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        self.args.push(&self.service);
        self.args.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute_stream(&self.args, sender).await
    }
}
crate::impl_builder_opts!(ComposeRunBuilder);
crate::impl_compose_opts!(ComposeRunBuilder);

// ── ComposeStartBuilder ───────────────────────────────────────────────────────

pub struct ComposeStartBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposeStartBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "start"]) }
    }
    pub fn service(mut self, s: impl Into<String>) -> Self { self.args.push(s.into()); self }
    
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ComposeStartBuilder);
crate::impl_compose_opts!(ComposeStartBuilder);

// ── ComposeStopBuilder ────────────────────────────────────────────────────────

pub struct ComposeStopBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposeStopBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "stop"]) }
    }
    pub fn service(mut self, s: impl Into<String>) -> Self { self.args.push(s.into()); self }
    pub fn timeout(mut self, t: u32)               -> Self { self.args.pair("--timeout", t.to_string()); self }
    
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ComposeStopBuilder);
crate::impl_compose_opts!(ComposeStopBuilder);

// ── ComposeRmBuilder ──────────────────────────────────────────────────────────

pub struct ComposeRmBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposeRmBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "rm"]) }
    }
    pub fn service(mut self, s: impl Into<String>) -> Self { self.args.push(s.into()); self }
    pub fn force(mut self)                         -> Self { self.args.flag("--force"); self }
    pub fn stop(mut self)                          -> Self { self.args.flag("--stop"); self }
    pub fn volumes(mut self)                       -> Self { self.args.flag("-v"); self }
    
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ComposeRmBuilder);
crate::impl_compose_opts!(ComposeRmBuilder);

// ── ComposeConfigBuilder ──────────────────────────────────────────────────────

pub struct ComposeConfigBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposeConfigBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "config"]) }
    }
    pub fn hash(mut self, h: impl Into<String>) -> Self { self.args.pair("--hash", h.into()); self }
    pub fn no_interpolate(mut self)             -> Self { self.args.flag("--no-interpolate"); self }
    pub fn quiet(mut self)                      -> Self { self.args.flag("--quiet"); self }
    pub fn services(mut self)                   -> Self { self.args.flag("--services"); self }
    pub fn volumes(mut self)                    -> Self { self.args.flag("--volumes"); self }
    
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
    
    pub async fn run_json(mut self) -> DockerResult<serde_json::Value> {
        self.args.pair("--format", "json");
        let output = self.cli.execute(&self.args).await?;
        let json = serde_json::from_str(&output.stdout).map_err(|e| crate::utils::exec::ExecError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        Ok(json)
    }
}
crate::impl_builder_opts!(ComposeConfigBuilder);
crate::impl_compose_opts!(ComposeConfigBuilder);

