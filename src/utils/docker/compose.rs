use super::{
    ComposeContainer, DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
};
use tokio::sync::mpsc;

impl DockerCli {
    pub async fn compose(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["compose"], args).await
    }
    pub async fn compose_up(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["compose", "up"], args).await
    }
    pub async fn compose_up_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut command = vec!["compose", "up"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn compose_logs_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut command = vec!["compose", "logs"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }

    pub async fn compose_down(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["compose", "down"], args).await
    }

    pub async fn compose_ps(&self, options: &[&str]) -> DockerResult<Vec<ComposeContainer>> {
        let mut args = vec!["compose", "ps", "--format", "json"];
        args.extend_from_slice(options);
        self.json(&args).await
    }
}
