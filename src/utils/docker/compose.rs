use super::{
    ComposeContainer, DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

impl DockerCli {
    /// Raw compose passthrough — prefer [`DockerCli::compose()`] DSL handle instead.
    pub async fn compose_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["compose"], args).await
    }
    pub async fn compose_raw_cancelled(
        &self,
        args: &[&str],
        cancel: &CancellationToken,
    ) -> DockerResult<DockerOutput> {
        self.prefixed_cancelled(&["compose"], args, cancel).await
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
