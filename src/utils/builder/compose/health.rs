use super::{
    compose::ComposeBuilder,
    spec::{ComposeRuntime, ComposeSpec},
};
use crate::utils::exec::{ExecError, ExecResult};
use tokio::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

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
                    self.docker
                        .stack_ps(&[
                            "--filter",
                            "desired-state=running",
                            "--format",
                            "{{json .}}",
                            spec.stack_name.as_str(),
                        ])
                        .await
                }
                ComposeRuntime::Compose => {
                    self.docker
                        .compose(&[
                            "--project-name",
                            spec.stack_name.as_str(),
                            "--env-file",
                            spec.env_file.as_str(),
                            "--file",
                            spec.compose_file_path().as_str(),
                            "ps",
                        ])
                        .await
                }
            };
            let output = match health_result {
                Ok(output) => output,
                Err(error)
                    if Instant::now() < deadline
                        && is_transient_docker_error(&error.to_string()) =>
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

fn is_transient_docker_error(message: &str) -> bool {
    let message = message.to_ascii_lowercase();
    [
        "cannot connect to the docker daemon",
        "docker daemon",
        "connection refused",
        "connection reset",
        "service unavailable",
        "temporarily unavailable",
        "context deadline exceeded",
    ]
    .iter()
    .any(|needle| message.contains(needle))
}
