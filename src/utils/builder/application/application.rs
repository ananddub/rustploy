use crate::utils::builder::application::traefik;
use crate::utils::builder::application::{stack::stack_spec, validation::validate_spec};
use crate::utils::builder::custom_type::{DeployEvent, DeployState, IdType};
use crate::utils::builder::hash_state::ApplicationState;
use crate::utils::builder::spec::{ApplicationSpec, BuilderEvent, DeploymentResult};
use crate::utils::builder::swarm::{ensure_overlay_network, ensure_swarm_manager};
use crate::utils::{
    docker::DockerCli,
    exec::{CommandExecutor, ExecError, ExecResult},
    paths::rustploy_paths,
};
use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration};
use tokio_util::sync::CancellationToken;
use crate::utils::builder::queue::queue::BuilderQueue;

#[derive(Clone, Debug)]
pub struct ApplicationBuilder {
    pub(super) executor: CommandExecutor,
    pub(super) docker: DockerCli,
    events: Option<mpsc::Sender<BuilderEvent>>,
    state: Option<(Arc<ApplicationState>, IdType)>,
    pub(super) health_timeout: Duration,
}

impl ApplicationBuilder {
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
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<DeploymentResult> {
        validate_spec(spec)?;

        self.emit(BuilderEvent::Preparing).await;
        self.cancelled(cancel)?;
        self.prepare_source(spec, cancel).await?;
        self.emit(BuilderEvent::SourceReady).await;

        self.cancelled(cancel)?;
        self.emit(BuilderEvent::Building).await;
        self.build_image(spec, cancel).await?;
        self.emit(BuilderEvent::ImageReady).await;

        self.cancelled(cancel)?;
        let paths = rustploy_paths();
        let app_dir = paths.application_dir(&spec.app_name);
        self.executor
            .run_cancelled("mkdir", ["-p", app_dir.as_str()], cancel)
            .await?;

        self.prepare_file_mounts(spec, cancel).await?;
        ensure_swarm_manager(&self.executor, &self.docker, cancel).await?;
        ensure_overlay_network(&self.docker, spec.network.as_str(), cancel).await?;

        let stack_file = format!("{app_dir}/stack.yml");
        let stack_yaml = serde_yaml::to_string(&stack_spec(spec))
            .map_err(|e| ExecError::Json(serde_json::Error::io(std::io::Error::other(e))))?;

        self.write_file_cancelled(&stack_file, stack_yaml.as_bytes(), cancel)
            .await?;

        self.emit(BuilderEvent::Deploying).await;
        if let Err(error) = self.docker.stacks().deploy(spec.stack_name.clone())
            .with_registry_auth()
            .compose_file(stack_file.as_str())
            .cancel_with(cancel.clone())
            .run()
            .await
        {
            self.docker.services().rollback(spec.service_name().as_str()).run().await?;
            self.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }

        self.cancelled(cancel)?;
        self.emit(BuilderEvent::Routing).await;

        self.emit(BuilderEvent::HealthCheck).await;
        if let Err(error) = self.wait_healthy(spec, cancel).await {
            self.docker.services().rollback(spec.service_name().as_str()).run().await?;
            self.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }

        self.emit(BuilderEvent::Deployed).await;
        Ok(DeploymentResult {
            app_name: spec.app_name.clone(),
            image: spec.image.clone(),
            service_name: spec.service_name(),
            stack_file,
        })
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

    async fn rollback_application(&self, spec: &ApplicationSpec, traefik_file: Option<&str>) {
        if let Some(path) = traefik_file {
            let _ = self.executor.run("rm", ["-f", path]).await;
        }
        let service = spec.service_name();
        if let Err(error) = self.docker.services().rollback(service.as_str()).run().await {
            tracing::warn!(service = %service, error = %error, "application rollback attempt failed");
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
        if let Some((state, id)) = &self.state
            && let Some(deploy_state) = crate::utils::builder::queue::common::builder_event_state_opt(&event)
        {
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

