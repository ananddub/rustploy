use super::{
    compose::ComposeBuilder,
    spec::{ComposeRuntime, ComposeSpec},
};
use crate::utils::exec::{ExecError, ExecResult};
use tokio::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;
use crate::utils::docker::query::filter::TaskFilter;

impl ComposeBuilder {
    pub(super) async fn wait_healthy(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let deadline = Instant::now() + self.health_timeout;
        loop {
            self.cancelled(cancel)?;
            let health_result = match spec.runtime {
                ComposeRuntime::Stack => {
                    self.docker.stacks().ps(&spec.stack_name)
                        .filter(TaskFilter::DesiredState(crate::utils::docker::query::filter::TaskDesiredState::Running))
                        .run()
                        .await
                }
                ComposeRuntime::Compose => {
                    self.docker.compose()
                        .ps()
                        .project(&spec.stack_name)
                        .env_file(&spec.env_file)
                        .file(&spec.compose_file_path())
                        .run()
                        .await
                }
            };
            let output = match health_result {
                Ok(output) => output,
                Err(error)
                    if Instant::now() < deadline
                        && crate::utils::docker::error::is_transient_docker_error(&error.to_string()) =>
                {
                    tracing::warn!(error = %error, "compose health check failed transiently; retrying");
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
                Err(error) => return Err(error),
            };
            if output.stdout.contains("Running") || output.stdout.contains("running") {
                return Ok(());
            }
            if output.stdout.contains("Rejected") || output.stdout.contains("Exit") {
                return Err(ExecError::CommandFailed {
                    code: None,
                    stderr: output.stdout,
                });
            }
            if Instant::now() >= deadline {
                return Err(ExecError::Timeout {
                    seconds: self.health_timeout.as_secs(),
                });
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
}

