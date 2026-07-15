use std::sync::Arc;

use auto_di::{resolve, singleton};
use sqlx::SqlitePool;

use crate::db::models::deployments::Deployment;
use crate::utils::builder::{
    custom_type::{ActiveDeploySnapshot, DeploySubscription, IdType},
    hash_state::ApplicationState,
};
use crate::utils::docker::{DockerCli, DockerStreamEvent};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelDeploymentResult {
    CancelRequested,
    NotRunning,
    NotCancellable,
    NotActiveInThisProcess,
}

#[derive(Debug, Clone, Default)]
pub struct DeploymentListFilter {
    pub status: Option<String>,
    pub state: Option<String>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub server_id: Option<i64>,
    pub limit: i64,
    pub offset: i64,
}

pub struct DeploymentService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl DeploymentService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Deployment> {
        sqlx::query_as!(
            Deployment,
            r#"SELECT id AS "id?", title, description, status, state, log_path, pid,
               error_message, is_preview_deployment, started_at, last_state_at, finished_at,
               application_id, compose_id, server_id, created_at
               FROM deployments WHERE id = ?"#,
            id,
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn list(&self, filter: DeploymentListFilter) -> sqlx::Result<Vec<Deployment>> {
        let status = normalize_filter_text(filter.status);
        let state = normalize_filter_text(filter.state);
        let limit = filter.limit.clamp(1, 200);
        let offset = filter.offset.max(0);

        sqlx::query_as!(
            Deployment,
            r#"SELECT id AS "id?", title, description, status, state, log_path, pid,
               error_message, is_preview_deployment, started_at, last_state_at, finished_at,
               application_id, compose_id, server_id, created_at
               FROM deployments
               WHERE (? IS NULL OR status = ?)
                 AND (? IS NULL OR state = ?)
                 AND (? IS NULL OR application_id = ?)
                 AND (? IS NULL OR compose_id = ?)
                 AND (? IS NULL OR server_id = ?)
               ORDER BY COALESCE(started_at, created_at) DESC, id DESC
               LIMIT ? OFFSET ?"#,
            status,
            status,
            state,
            state,
            filter.application_id,
            filter.application_id,
            filter.compose_id,
            filter.compose_id,
            filter.server_id,
            filter.server_id,
            limit,
            offset,
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn list_running(&self, limit: i64, offset: i64) -> sqlx::Result<Vec<Deployment>> {
        self.list(DeploymentListFilter {
            status: Some("RUNNING".into()),
            limit,
            offset,
            ..Default::default()
        })
        .await
    }

    pub async fn cancel(&self, deployment_id: i64) -> sqlx::Result<CancelDeploymentResult> {
        let (application_id, compose_id, status) =
            sqlx::query_as::<_, (Option<i64>, Option<i64>, String)>(
                "SELECT application_id, compose_id, status FROM deployments WHERE id = ?",
            )
            .bind(deployment_id)
            .fetch_one(self.db.as_ref())
            .await?;

        if status == "QUEUED" {
            sqlx::query(
                "UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE id = ? AND status = 'QUEUED'",
            )
            .bind(deployment_id)
            .execute(self.db.as_ref())
            .await?;
            return Ok(CancelDeploymentResult::CancelRequested);
        }

        if status != "RUNNING" {
            return Ok(CancelDeploymentResult::NotRunning);
        }

        let Some(target_id) = application_id
            .map(IdType::AppId)
            .or_else(|| compose_id.map(IdType::ComposeId))
        else {
            return Ok(CancelDeploymentResult::NotCancellable);
        };

        let state = resolve::<ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        if !state.cancel_by_id(target_id) {
            return Ok(CancelDeploymentResult::NotActiveInThisProcess);
        }

        sqlx::query(
            "UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE id = ? AND status = 'RUNNING'",
        )
        .bind(deployment_id)
        .execute(self.db.as_ref())
        .await?;

        Ok(CancelDeploymentResult::CancelRequested)
    }

    pub async fn list_active_components(&self) -> sqlx::Result<Vec<ActiveDeploySnapshot>> {
        let state = resolve::<ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(state.active_deployments())
    }

    pub async fn subscribe_component(
        &self,
        component_id: IdType,
    ) -> sqlx::Result<Option<DeploySubscription>> {
        let state = resolve::<ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(state.subscribe(component_id))
    }

    pub async fn stream_docker_container_logs(
        &self,
        server_id: Option<i64>,
        target: String,
        options: Vec<String>,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        Ok(spawn_docker_stream(
            docker,
            docker_logs_command("container", target, options),
        ))
    }

    pub async fn stream_docker_container_stats(
        &self,
        server_id: Option<i64>,
        target: String,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        let mut command = vec![
            "container".into(),
            "stats".into(),
            "--format".into(),
            "{{json .}}".into(),
        ];
        if !stream {
            command.push("--no-stream".into());
        }
        command.push(target);
        Ok(spawn_docker_stream(docker, command))
    }

    pub async fn stream_application_stats(
        &self,
        application_id: i64,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let (app_name, server_id) = sqlx::query_as::<_, (String, Option<i64>)>(
            "SELECT app_name, server_id FROM applications WHERE id = ?",
        )
        .bind(application_id)
        .fetch_one(self.db.as_ref())
        .await?;

        let docker = self.docker_for_server(server_id).await?;
        let service_name = format!("{app_name}_{app_name}");
        let filter = crate::utils::docker::query::filter::ContainerFilter::Label(
            "com.docker.swarm.service.name".to_string(),
            service_name,
        );
        let containers = docker
            .containers()
            .ps()
            .filter(filter)
            .list()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let targets = containers
            .into_iter()
            .map(|container| container.id)
            .filter(|id| !id.trim().is_empty())
            .collect::<Vec<_>>();

        Ok(spawn_stats_stream(docker, targets, stream))
    }

    pub async fn stream_compose_stats(
        &self,
        compose_id: i64,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let (app_name, server_id) = sqlx::query_as::<_, (String, Option<i64>)>(
            "SELECT app_name, server_id FROM compose_projects WHERE id = ?",
        )
        .bind(compose_id)
        .fetch_one(self.db.as_ref())
        .await?;

        let docker = self.docker_for_server(server_id).await?;
        let filter = crate::utils::docker::query::filter::ContainerFilter::Label(
            "com.docker.compose.project".to_string(),
            app_name,
        );
        let containers = docker
            .containers()
            .ps()
            .filter(filter)
            .list()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let targets = containers
            .into_iter()
            .map(|container| container.id)
            .filter(|id| !id.trim().is_empty())
            .collect::<Vec<_>>();

        Ok(spawn_stats_stream(docker, targets, stream))
    }

    pub async fn stream_global_stats(
        &self,
        server_id: Option<i64>,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        Ok(spawn_stats_stream(docker, Vec::new(), stream))
    }

    pub async fn stream_docker_service_logs(
        &self,
        server_id: Option<i64>,
        target: String,
        options: Vec<String>,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        Ok(spawn_docker_stream(
            docker,
            docker_logs_command("service", target, options),
        ))
    }

    pub async fn stream_docker_compose_logs(
        &self,
        server_id: Option<i64>,
        args: Vec<String>,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        Ok(spawn_docker_stream(docker, args))
    }

    async fn docker_for_server(&self, server_id: Option<i64>) -> sqlx::Result<DockerCli> {
        match server_id {
            Some(server_id) => {
                let executor =
                    crate::services::compose::remote::remote_executor(self.db.as_ref(), server_id)
                        .await
                        .map_err(sqlx::Error::Protocol)?;
                Ok(DockerCli::from_remote_executor(executor))
            }
            None => Ok(DockerCli::new_local()),
        }
    }
}

fn normalize_filter_text(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_ascii_uppercase())
        .filter(|value| !value.is_empty())
}

fn spawn_stats_stream(
    docker: DockerCli,
    targets: Vec<String>,
    stream: bool,
) -> mpsc::Receiver<DockerStreamEvent> {
    let mut command = vec![
        "container".into(),
        "stats".into(),
        "--format".into(),
        "{{json .}}".into(),
    ];
    if !stream {
        command.push("--no-stream".into());
    }
    command.extend(targets);
    spawn_docker_stream(docker, command)
}

fn docker_logs_command(kind: &str, target: String, options: Vec<String>) -> Vec<String> {
    let mut command = vec![kind.into(), "logs".into()];
    command.extend(options);
    command.push(target);
    command
}

fn spawn_docker_stream(
    docker: DockerCli,
    command: Vec<String>,
) -> mpsc::Receiver<DockerStreamEvent> {
    let (sender, receiver) = mpsc::channel(128);
    let error_sender = sender.clone();

    tokio::spawn(async move {
        if let Err(error) = docker.run_stream(command, sender).await {
            let _ = error_sender
                .send(DockerStreamEvent::Stderr(
                    format!("docker stream failed: {error}\n").into_bytes(),
                ))
                .await;
        }
    });

    receiver
}
