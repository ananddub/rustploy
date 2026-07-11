use super::{
    labels::write_labeled_compose,
    spec::{ComposeDeploymentResult, ComposeRuntime, ComposeSpec},
    validation::validate_spec,
};
use crate::utils::{
    builder::{
        custom_type::{DeployState, IdType},
        hash_state::ApplicationState,
        spec::BuilderEvent,
    },
    docker::DockerCli,
    exec::{CommandExecutor, ExecError, ExecResult},
};
use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct ComposeBuilder {
    pub(super) executor: CommandExecutor,
    pub(super) docker: DockerCli,
    events: Option<mpsc::Sender<BuilderEvent>>,
    state: Option<(Arc<ApplicationState>, IdType)>,
    pub(super) health_timeout: Duration,
}

impl ComposeBuilder {
    pub fn new(executor: CommandExecutor) -> Self {
        Self {
            docker: DockerCli::from_executor(executor.clone()),
            executor,
            events: None,
            state: None,
            health_timeout: Duration::from_secs(120),
        }
    }

    pub fn with_events(mut self, events: mpsc::Sender<BuilderEvent>) -> Self {
        self.events = Some(events);
        self
    }

    pub fn with_state(mut self, state: Arc<ApplicationState>, id: IdType) -> Self {
        self.state = Some((state, id));
        self
    }

    pub fn with_health_timeout(mut self, timeout: Duration) -> Self {
        self.health_timeout = timeout;
        self
    }

    pub async fn deploy(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<ComposeDeploymentResult> {
        validate_spec(spec)?;
        self.emit(BuilderEvent::Preparing).await;
        self.cancelled(cancel)?;
        self.prepare_source(spec).await?;
        self.emit(BuilderEvent::SourceReady).await;
        self.prepare_runtime_files(spec).await?;
        write_labeled_compose(self, spec).await?;

        self.emit(BuilderEvent::Deploying).await;
        match spec.runtime {
            ComposeRuntime::Stack => self.deploy_stack(spec).await?,
            ComposeRuntime::Compose => self.deploy_compose(spec).await?,
        }

        self.emit(BuilderEvent::Routing).await;

        self.emit(BuilderEvent::HealthCheck).await;
        if let Err(error) = self.wait_healthy(spec, cancel).await {
            self.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }

        self.emit(BuilderEvent::Deployed).await;
        Ok(ComposeDeploymentResult {
            app_name: spec.app_name.clone(),
            stack_name: spec.stack_name.clone(),
            compose_file: spec.compose_file_path(),
        })
    }

    pub async fn stop(&self, spec: &ComposeSpec) -> ExecResult<()> {
        match spec.runtime {
            ComposeRuntime::Stack => {
                self.docker
                    .stack_remove(&[spec.stack_name.as_str()])
                    .await?;
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
                        "down",
                    ])
                    .await?;
            }
        }
        self.emit(BuilderEvent::Cancelled).await;
        Ok(())
    }

    async fn deploy_stack(&self, spec: &ComposeSpec) -> ExecResult<()> {
        self.executor
            .run(
                "sh",
                [
                    "-c",
                    "docker compose --env-file \"$1\" --file \"$2\" config > \"$3\" && docker stack deploy --compose-file \"$3\" --with-registry-auth \"$4\"",
                    "rustploy-compose-stack",
                    spec.env_file.as_str(),
                    spec.compose_file_path().as_str(),
                    spec.rendered_stack_file.as_str(),
                    spec.stack_name.as_str(),
                ],
            )
            .await?;
        Ok(())
    }

    async fn deploy_compose(&self, spec: &ComposeSpec) -> ExecResult<()> {
        self.docker
            .compose(&[
                "--project-name",
                spec.stack_name.as_str(),
                "--env-file",
                spec.env_file.as_str(),
                "--file",
                spec.compose_file_path().as_str(),
                "up",
                "--detach",
            ])
            .await?;
        Ok(())
    }

    pub(super) async fn write_file(&self, path: &str, content: &[u8]) -> ExecResult<()> {
        self.executor
            .run_with_stdin(
                "sh",
                ["-c", "umask 077; cat > \"$1\"", "rustploy-write", path],
                content,
            )
            .await?;
        Ok(())
    }

    pub(super) fn cancelled(&self, token: &CancellationToken) -> ExecResult<()> {
        if token.is_cancelled() {
            Err(ExecError::StreamCancelled)
        } else {
            Ok(())
        }
    }

    async fn emit(&self, event: BuilderEvent) {
        if let Some(sender) = &self.events {
            let _ = sender.send(event.clone()).await;
        }
        if let Some((state, id)) = &self.state {
            let deploy_state = match event {
                BuilderEvent::Preparing => DeployState::Preparing,
                BuilderEvent::SourceReady => DeployState::GitSuccess,
                BuilderEvent::Building => DeployState::Building,
                BuilderEvent::ImageReady => DeployState::BuildSuccess,
                BuilderEvent::Deploying | BuilderEvent::Routing => DeployState::Deploying,
                BuilderEvent::HealthCheck => DeployState::HealthCheck,
                BuilderEvent::Deployed => DeployState::Deployed,
                BuilderEvent::Cancelled => DeployState::StoppedByUser,
                BuilderEvent::Failed(error) => DeployState::Failed(error),
            };
            let _ = state.send_state(id.clone(), deploy_state);
        }
    }
}
