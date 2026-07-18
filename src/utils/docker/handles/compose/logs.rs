use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct ComposeLogsBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ComposeLogsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, service: impl Into<String>) -> Self {
        let mut args = ArgBuilder::cmd(&["compose", "logs"]);
        args.push(service.into());
        Self { cli, args }
    }
    pub fn follow(mut self) -> Self {
        self.args.flag("--follow");
        self
    }
    pub fn tail(mut self, n: usize) -> Self {
        self.args.pair("--tail", n.to_string());
        self
    }
    pub fn timestamps(mut self) -> Self {
        self.args.flag("--timestamps");
        self
    }
    pub fn no_color(mut self) -> Self {
        self.args.flag("--no-color");
        self
    }
    pub fn since(mut self, s: impl Into<String>) -> Self {
        self.args.pair("--since", s.into());
        self
    }
    pub fn until(mut self, u: impl Into<String>) -> Self {
        self.args.pair("--until", u.into());
        self
    }
    pub fn print(&self) -> String {
        self.args.preview()
    }
    pub async fn stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        self.cli.execute_stream(&self.args, sender).await
    }
    /// Specify an alternate compose file
    pub fn file(mut self, f: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--file", f.into());
        self
    }

    /// Specify an alternate environment file
    pub fn env_file(mut self, f: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--env-file", f.into());
        self
    }

    /// Specify an alternate project name
    pub fn project(mut self, p: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--project-name", p.into());
        self
    }

    /// Specify a profile to enable
    pub fn profile(mut self, p: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--profile", p.into());
        self
    }
}

crate::impl_builder_opts!(ComposeLogsBuilder);

impl crate::utils::exec::script::IntoCommand for ComposeLogsBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
