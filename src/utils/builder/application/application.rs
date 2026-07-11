use crate::utils::builder::application::traefik;
use crate::utils::builder::application::{stack::stack_spec, validation::validate_spec};
use crate::utils::builder::custom_type::{DeployState, IdType};
use crate::utils::builder::hash_state::ApplicationState;
use crate::utils::builder::spec::{ApplicationSpec, BuilderEvent, DeploymentResult};
use crate::utils::{
    docker::DockerCli,
    exec::{CommandExecutor, ExecError, ExecResult},
};
use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration};
use tokio_util::sync::CancellationToken;

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
        self.prepare_source(spec).await?;
        self.emit(BuilderEvent::SourceReady).await;

        self.cancelled(cancel)?;
        self.emit(BuilderEvent::Building).await;
        self.build_image(spec).await?;
        self.emit(BuilderEvent::ImageReady).await;

        self.cancelled(cancel)?;
        let app_dir = format!("/etc/rustploy/applications/{}", spec.app_name);
        self.executor.run("mkdir", ["-p", app_dir.as_str()]).await?;
        self.prepare_file_mounts(spec).await?;

        let stack_file = format!("{app_dir}/stack.yml");
        let stack_yaml = serde_yaml::to_string(&stack_spec(spec))
            .map_err(|e| ExecError::Json(serde_json::Error::io(std::io::Error::other(e))))?;
        self.write_file(&stack_file, stack_yaml.as_bytes()).await?;

        self.emit(BuilderEvent::Deploying).await;
        self.docker
            .stack_deploy(&[
                "--compose-file",
                stack_file.as_str(),
                "--with-registry-auth",
                spec.stack_name.as_str(),
            ])
            .await?;

        self.cancelled(cancel)?;
        let traefik_file = format!("/etc/rustploy/traefik/dynamic/{}.json", spec.app_name);
        let routing = serde_json::to_vec_pretty(&traefik::application_config(spec))?;
        self.write_file(&traefik_file, &routing).await?;
        self.emit(BuilderEvent::Routing).await;

        self.emit(BuilderEvent::HealthCheck).await;
        if let Err(error) = self.wait_healthy(spec, cancel).await {
            let _ = self.executor.run("rm", ["-f", traefik_file.as_str()]).await;
            self.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }

        self.emit(BuilderEvent::Deployed).await;
        Ok(DeploymentResult {
            app_name: spec.app_name.clone(),
            image: spec.image.clone(),
            service_name: spec.service_name(),
            stack_file,
            traefik_file,
        })
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
        if let Some((state, id)) = &self.state
            && let Some(deploy_state) = builder_event_state(&event)
        {
            let _ = state.send_state(id.clone(), deploy_state);
        }
    }
}

fn builder_event_state(event: &BuilderEvent) -> Option<DeployState> {
    Some(match event {
        BuilderEvent::Preparing => DeployState::Preparing,
        BuilderEvent::SourceReady => DeployState::GitSuccess,
        BuilderEvent::Building => DeployState::Building,
        BuilderEvent::ImageReady => DeployState::BuildSuccess,
        BuilderEvent::Deploying => DeployState::Deploying,
        BuilderEvent::Routing => DeployState::Deploying,
        BuilderEvent::HealthCheck => DeployState::HealthCheck,
        BuilderEvent::Deployed => DeployState::Deployed,
        BuilderEvent::Cancelled => DeployState::Cancelled,
        BuilderEvent::Failed(error) => DeployState::Failed(error.clone()),
    })
}
