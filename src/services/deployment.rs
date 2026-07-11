use std::sync::Arc;

use auto_di::{resolve, singleton};
use sqlx::SqlitePool;

use crate::utils::builder::{custom_type::IdType, hash_state::ApplicationState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelDeploymentResult {
    CancelRequested,
    NotRunning,
    NotCancellable,
    NotActiveInThisProcess,
}

pub struct DeploymentService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl DeploymentService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn cancel(&self, deployment_id: i64) -> sqlx::Result<CancelDeploymentResult> {
        let (application_id, compose_id, status) =
            sqlx::query_as::<_, (Option<i64>, Option<i64>, String)>(
                "SELECT application_id, compose_id, status FROM deployments WHERE id = ?",
            )
            .bind(deployment_id)
            .fetch_one(self.db.as_ref())
            .await?;

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
}
