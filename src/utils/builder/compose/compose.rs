use super::{
    labels::write_labeled_compose,
    spec::{ComposeDeploymentResult, ComposeRuntime, ComposeSpec},
    validation::validate_spec,
};
use crate::utils::{
    builder::{
        custom_type::{DeployEvent, DeployState, IdType},
        hash_state::ApplicationState,
        spec::BuilderEvent,
        swarm::{ensure_overlay_network, ensure_swarm_manager, RUSTPLOY_NETWORK},
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
        self.prepare_source(spec, cancel).await?;
        self.emit(BuilderEvent::SourceReady).await;
        self.prepare_runtime_files(spec, cancel).await?;
        write_labeled_compose(self, spec, cancel).await?;

        self.emit(BuilderEvent::Deploying).await;
        let deploy_result = match spec.runtime {
            ComposeRuntime::Stack => self.deploy_stack(spec, cancel).await,
            ComposeRuntime::Compose => self.deploy_compose(spec, cancel).await,
        };
        if let Err(error) = deploy_result {
            self.cleanup_failed_deploy(spec).await;
            self.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }

        self.emit(BuilderEvent::Routing).await;

        self.emit(BuilderEvent::HealthCheck).await;
        if let Err(error) = self.wait_healthy(spec, cancel).await {
            self.cleanup_failed_deploy(spec).await;
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
                self.docker.stack_remove_raw(&[&spec.stack_name]).await?;
            }
            ComposeRuntime::Compose => {
                self.docker
                    .compose_raw(&[
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

    async fn deploy_stack(&self, spec: &ComposeSpec, cancel: &CancellationToken) -> ExecResult<()> {
        ensure_swarm_manager(&self.executor, &self.docker, cancel).await?;
        ensure_overlay_network(&self.docker, RUSTPLOY_NETWORK, cancel).await?;
        self.emit(BuilderEvent::Message(format!(
            "building compose stack {} from {}",
            spec.stack_name,
            spec.compose_file_path()
        )))
        .await;
        self.run_with_retry(
            "sh",
            &[
                "-c",
                "docker compose --env-file \"$1\" --file \"$2\" build && docker compose --env-file \"$1\" --file \"$2\" config > \"$3\" && docker stack deploy --compose-file \"$3\" --with-registry-auth --resolve-image never \"$4\"",
                "rustploy-compose-stack",
                spec.env_file.as_str(),
                spec.compose_file_path().as_str(),
                spec.rendered_stack_file.as_str(),
                spec.stack_name.as_str(),
            ],
            cancel,
        )
        .await
    }

    async fn deploy_compose(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        self.emit(BuilderEvent::Message(format!(
            "docker compose build project {} file {}",
            spec.stack_name,
            spec.compose_file_path()
        )))
        .await;
        self.docker_compose_with_retry(
            &[
                "--project-name",
                spec.stack_name.as_str(),
                "--env-file",
                spec.env_file.as_str(),
                "--file",
                spec.compose_file_path().as_str(),
                "build",
            ],
            cancel,
        )
        .await?;
        self.emit(BuilderEvent::Message(format!(
            "docker compose up project {} file {}",
            spec.stack_name,
            spec.compose_file_path()
        )))
        .await;
        self.docker_compose_with_retry(
            &[
                "--project-name",
                spec.stack_name.as_str(),
                "--env-file",
                spec.env_file.as_str(),
                "--file",
                spec.compose_file_path().as_str(),
                "up",
                "--detach",
            ],
            cancel,
        )
        .await
    }

    pub(super) async fn write_file_cancelled(
        &self,
        path: &str,
        content: &[u8],
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        self.executor
            .run_with_stdin_cancelled(
                "sh",
                ["-c", "umask 077; cat > \"$1\"", "rustploy-write", path],
                content,
                cancel,
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

    async fn docker_compose_with_retry(
        &self,
        args: &[&str],
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            self.cancelled(cancel)?;
            match self.docker.compose_raw_cancelled(args, cancel).await {
                Ok(_) => return Ok(()),
                Err(error) if attempts < 4 && is_transient_docker_error(&error.to_string()) => {
                    tracing::warn!(attempts, error = %error, "docker compose command failed transiently; retrying");
                    tokio::time::sleep(Duration::from_secs(2 * attempts)).await;
                }
                Err(error) => return Err(error),
            }
        }
    }

    async fn run_with_retry(
        &self,
        program: &str,
        args: &[&str],
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            self.cancelled(cancel)?;
            match self.executor.run_cancelled(program, args, cancel).await {
                Ok(_) => return Ok(()),
                Err(error) if attempts < 4 && is_transient_docker_error(&error.to_string()) => {
                    tracing::warn!(attempts, error = %error, "docker command failed transiently; retrying");
                    tokio::time::sleep(Duration::from_secs(2 * attempts)).await;
                }
                Err(error) => return Err(error),
            }
        }
    }

    async fn cleanup_failed_deploy(&self, spec: &ComposeSpec) {
        match spec.runtime {
            ComposeRuntime::Stack => {
                if let Err(error) = self.docker.stack_remove_raw(&[&spec.stack_name]).await {
                    tracing::warn!(stack = %spec.stack_name, error = %error, "compose stack cleanup failed");
                }
            }
            ComposeRuntime::Compose => {
                if let Err(error) = self
                    .docker
                    .compose_raw(&[
                        "--project-name",
                        spec.stack_name.as_str(),
                        "--env-file",
                        spec.env_file.as_str(),
                        "--file",
                        spec.compose_file_path().as_str(),
                        "down",
                    ])
                    .await
                {
                    tracing::warn!(compose = %spec.stack_name, error = %error, "compose cleanup failed");
                }
            }
        }
    }

    pub(super) async fn emit(&self, event: BuilderEvent) {
        if let Some(sender) = &self.events {
            let _ = sender.send(event.clone()).await;
        }
        if let Some((state, id)) = &self.state
            && let BuilderEvent::Message(message) = &event
        {
            if let Some(sender) = state.get_broadcast_send(id.clone()) {
                let _ = sender.send(DeployEvent::Message(message.clone()));
            }
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
                BuilderEvent::Message(_) => return,
                BuilderEvent::Failed(error) => DeployState::Failed(error),
                BuilderEvent::RecoverAfterRestart => DeployState::RecoverAfterRestart,
            };
            let _ = state.send_state(id.clone(), deploy_state);
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
