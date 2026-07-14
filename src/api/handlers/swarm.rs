use std::sync::Arc;

use auto_route::controller;
use axum::{Json, http::StatusCode};
use sqlx::SqlitePool;

use crate::{
    api::dto::swarm::{
        NodeActionDto, NodeAvailabilityDto, NodeDto, SwarmConnectionDto, SwarmInfoDto,
        SwarmTokensDto,
    },
    utils::{
        docker::DockerCli,
        exec::{CommandExecutor, ExecError, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey},
        jwt::claim::Claims,
    },
};

type ApiError = (StatusCode, String);

pub struct SwarmController {
    db: Arc<SqlitePool>,
}

#[controller("/swarm")]
impl SwarmController {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    /// GET /swarm/info — swarm state + node count.
    #[post("/info")]
    async fn info(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<Json<SwarmInfoDto>, ApiError> {
        let docker = self.docker(body.server_id).await?;
        let raw = docker
            .run(["info", "--format", "{{json .Swarm}}"])
            .await
            .map_err(map_exec)?;

        let v: serde_json::Value =
            serde_json::from_str(raw.stdout_trimmed()).map_err(|e| internal(e.to_string()))?;

        Ok(Json(SwarmInfoDto {
            node_id: str_field(&v, "NodeID"),
            node_addr: str_field(&v, "NodeAddr"),
            local_node_state: str_field(&v, "LocalNodeState"),
            control_available: v
                .get("ControlAvailable")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            nodes: v
                .get("Nodes")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            managers: v
                .get("Managers")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
        }))
    }

    /// POST /swarm/tokens — get worker + manager join tokens.
    #[post("/tokens")]
    async fn tokens(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<Json<SwarmTokensDto>, ApiError> {
        let docker = self.docker(body.server_id).await?;

        let worker = docker
            .swarm_join_token_raw(&["--quiet", "worker"])
            .await
            .map_err(map_exec)?;
        let manager = docker
            .swarm_join_token_raw(&["--quiet", "manager"])
            .await
            .map_err(map_exec)?;

        Ok(Json(SwarmTokensDto {
            worker: worker.stdout_trimmed().into(),
            manager: manager.stdout_trimmed().into(),
        }))
    }

    /// POST /swarm/nodes — list all nodes.
    #[post("/nodes")]
    async fn nodes(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<Json<Vec<NodeDto>>, ApiError> {
        let docker = self.docker(body.server_id).await?;
        let nodes = docker.nodes_raw(&[]).await.map_err(map_exec)?;
        Ok(Json(nodes.into_iter().map(NodeDto::from).collect()))
    }

    #[post("/nodes/promote")]
    async fn promote(
        &self,
        _claims: Claims,
        Json(body): Json<NodeActionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .node_promote_raw(&[body.node_id.as_str()])
            .await
            .map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/nodes/demote")]
    async fn demote(
        &self,
        _claims: Claims,
        Json(body): Json<NodeActionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .node_demote_raw(&[body.node_id.as_str()])
            .await
            .map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/nodes/availability")]
    async fn set_availability(
        &self,
        _claims: Claims,
        Json(body): Json<NodeAvailabilityDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .node_update_raw(&[
                "--availability",
                body.availability.to_ascii_lowercase().as_str(),
                body.node_id.as_str(),
            ])
            .await
            .map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/nodes/remove")]
    async fn remove_node(
        &self,
        _claims: Claims,
        Json(body): Json<NodeActionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .node_remove_raw(&["--force", body.node_id.as_str()])
            .await
            .map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    /// POST /swarm/leave — make a node leave the swarm.
    #[post("/leave")]
    async fn leave(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker.swarm_leave_raw(true).await.map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    // ---------------------------------------------------------------- //
    //  Helpers                                                          //
    // ---------------------------------------------------------------- //

    async fn docker(&self, server_id: Option<i64>) -> Result<DockerCli, ApiError> {
        match server_id {
            None => Ok(DockerCli::from_executor(CommandExecutor::Local(
                LocalExecutor::new(),
            ))),
            Some(id) => {
                let executor = remote_executor_for(self.db.as_ref(), id).await?;
                Ok(DockerCli::from_remote_executor(executor))
            }
        }
    }
}

/// Build a RemoteExecutor from DB server+ssh_key record.
async fn remote_executor_for(db: &SqlitePool, server_id: i64) -> Result<RemoteExecutor, ApiError> {
    let row = sqlx::query_as::<_, (String, i64, String, String, String)>(
        r#"SELECT s.ip_address, s.port, s.username, k.private_key, k.public_key
           FROM servers s JOIN ssh_keys k ON k.id = s.ssh_key_id
           WHERE s.id = ?"#,
    )
    .bind(server_id)
    .fetch_one(db)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "server or SSH key not found".into()))?;

    let port = u16::try_from(row.1)
        .map_err(|_| (StatusCode::BAD_REQUEST, "invalid SSH port".into()))?;

    Ok(RemoteExecutor::new(
        row.0,
        port,
        row.2,
        SshAuth::key_pair(row.3, row.4),
        SshHostKey::InsecureAcceptAny,
    )
    .with_pool_size(2)
    .with_sudo())
}

fn str_field(v: &serde_json::Value, key: &str) -> String {
    v.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_owned()
}

fn map_exec(error: ExecError) -> ApiError {
    tracing::error!(error = %error, "swarm command failed");
    match error {
        ExecError::CommandFailed { .. } => (StatusCode::BAD_GATEWAY, error.to_string()),
        ExecError::Ssh(_) => (StatusCode::BAD_GATEWAY, error.to_string()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
    }
}

fn internal(msg: String) -> ApiError {
    (StatusCode::INTERNAL_SERVER_ERROR, msg)
}
