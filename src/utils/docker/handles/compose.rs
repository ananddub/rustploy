use crate::utils::docker::{
    core::ArgBuilder,
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent, ComposeContainer,
};
use tokio::sync::mpsc;

pub struct ComposeHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> ComposeHandle<'a> {
    pub fn up(&self)                               -> UpBuilder<'_>   { UpBuilder::new(self.0) }
    pub fn down(&self)                             -> DownBuilder<'_> { DownBuilder::new(self.0) }
    pub fn logs(&self, service: impl Into<String>) -> ComposeLogsBuilder<'_> { ComposeLogsBuilder::new(self.0, service) }
    pub fn ps(&self)                               -> ComposePsBuilder<'_> { ComposePsBuilder::new(self.0) }
    pub fn restart(&self, services: impl IntoIterator<Item = impl Into<String>>) -> ComposeRestartBuilder<'_> {
        ComposeRestartBuilder::new(self.0, services)
    }
}

fn compose_base(args: &ArgBuilder) -> String { args.preview() }

// ── UpBuilder ─────────────────────────────────────────────────────────────────

pub struct UpBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> UpBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "up"]) }
    }
    pub fn file(mut self, f: impl Into<String>)         -> Self { self.args.pair("--file", f.into()); self }
    pub fn project(mut self, p: impl Into<String>)      -> Self { self.args.pair("--project-name", p.into()); self }
    pub fn service(mut self, s: impl Into<String>)      -> Self { self.args.push(s.into()); self }
    pub fn detach(mut self)                              -> Self { self.args.flag("--detach"); self }
    pub fn build(mut self)                               -> Self { self.args.flag("--build"); self }
    pub fn no_deps(mut self)                             -> Self { self.args.flag("--no-deps"); self }
    pub fn remove_orphans(mut self)                      -> Self { self.args.flag("--remove-orphans"); self }
    pub fn scale(mut self, service: impl AsRef<str>, n: u32) -> Self {
        self.args.pair("--scale", format!("{}={}", service.as_ref(), n)); self
    }
    pub fn print(&self)  -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let a = self.args.build();
        let refs: Vec<&str> = a.iter().map(String::as_str).collect();
        self.cli.run(refs).await
    }
    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        let a = self.args.build();
        let refs: Vec<&str> = a.iter().map(String::as_str).collect();
        self.cli.run_stream(refs, sender).await
    }
}

// ── DownBuilder ───────────────────────────────────────────────────────────────

pub struct DownBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> DownBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "down"]) }
    }
    pub fn file(mut self, f: impl Into<String>)    -> Self { self.args.pair("--file", f.into()); self }
    pub fn project(mut self, p: impl Into<String>) -> Self { self.args.pair("--project-name", p.into()); self }
    pub fn volumes(mut self)                        -> Self { self.args.flag("--volumes"); self }
    pub fn remove_orphans(mut self)                 -> Self { self.args.flag("--remove-orphans"); self }
    pub fn rmi(mut self, kind: impl Into<String>)  -> Self { self.args.pair("--rmi", kind.into()); self }
    pub fn print(&self)  -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let a = self.args.build();
        let refs: Vec<&str> = a.iter().map(String::as_str).collect();
        self.cli.run(refs).await
    }
}

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
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn stream(self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        let a = self.args.build();
        let refs: Vec<&str> = a.iter().map(String::as_str).collect();
        self.cli.run_stream(refs, sender).await
    }
}

// ── ComposePsBuilder ──────────────────────────────────────────────────────────

pub struct ComposePsBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposePsBuilder<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["compose", "ps", "--format", "json"]) }
    }
    pub fn all(mut self)                          -> Self { self.args.flag("--all"); self }
    pub fn service(mut self, s: impl Into<String>) -> Self { self.args.push(s.into()); self }
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn list(self) -> DockerResult<Vec<ComposeContainer>> {
        let a = self.args.build();
        let refs: Vec<&str> = a.iter().map(String::as_str).collect();
        self.cli.json(&refs).await
    }
}

// ── ComposeRestartBuilder ─────────────────────────────────────────────────────

pub struct ComposeRestartBuilder<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> ComposeRestartBuilder<'a> {
    fn new(cli: &'a DockerCli, services: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut args = ArgBuilder::cmd(&["compose", "restart"]);
        for s in services { args.push(s.into()); }
        Self { cli, args }
    }
    pub fn file(mut self, f: impl Into<String>) -> Self { self.args.pair("--file", f.into()); self }
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let a = self.args.build();
        let refs: Vec<&str> = a.iter().map(String::as_str).collect();
        self.cli.run(refs).await
    }
}
