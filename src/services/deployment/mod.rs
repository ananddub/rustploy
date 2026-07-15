use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::db::models::deployments::Deployment;
use crate::utils::builder::custom_type::IdType;

pub mod docker;
pub mod log;

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

        let state = auto_di::resolve::<crate::utils::builder::hash_state::ApplicationState>()
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

    pub async fn list_active_components(&self) -> sqlx::Result<Vec<crate::utils::builder::custom_type::ActiveDeploySnapshot>> {
        let state = auto_di::resolve::<crate::utils::builder::hash_state::ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(state.active_deployments())
    }

    pub async fn subscribe_component(
        &self,
        component_id: IdType,
    ) -> sqlx::Result<Option<crate::utils::builder::custom_type::DeploySubscription>> {
        let state = auto_di::resolve::<crate::utils::builder::hash_state::ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(state.subscribe(component_id))
    }
}

fn normalize_filter_text(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_ascii_uppercase())
        .filter(|value| !value.is_empty())
}
