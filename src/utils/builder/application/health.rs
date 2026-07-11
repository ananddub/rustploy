use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::ApplicationSpec,
    exec::{ExecError, ExecResult},
};
use tokio::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

impl ApplicationBuilder {
    pub(super) async fn wait_healthy(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let deadline = Instant::now() + self.health_timeout;
        loop {
            self.cancelled(cancel)?;
            let output = self
                .docker
                .run([
                    "service",
                    "ps",
                    "--filter",
                    "desired-state=running",
                    "--format",
                    "{{json .}}",
                    spec.service_name().as_str(),
                ])
                .await?;
            let rows = output
                .stdout
                .lines()
                .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
                .collect::<Vec<_>>();
            if rows.iter().any(|row| {
                row.get("CurrentState")
                    .and_then(|value| value.as_str())
                    .is_some_and(|value| value.starts_with("Running"))
            }) {
                return Ok(());
            }
            if let Some(error) = rows
                .iter()
                .filter_map(|row| row.get("Error").and_then(|value| value.as_str()))
                .find(|error| !error.is_empty())
            {
                return Err(ExecError::CommandFailed {
                    code: None,
                    stderr: error.into(),
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
