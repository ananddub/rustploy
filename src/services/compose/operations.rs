use auto_di::resolve;
use sqlx::Row;
use uuid::Uuid;

use crate::utils::builder::{custom_type::IdType, hash_state::ApplicationState};

use super::{
    ComposeOperation, ComposeOperationResult, ComposeRecord, ComposeService,
    runtime::{execute_operation, is_cancelled_error},
};

impl ComposeService {
    pub async fn run_operation(
        &self,
        id: i64,
        operation: ComposeOperation,
    ) -> sqlx::Result<ComposeOperationResult> {
        let mut tx = self.db.begin().await?;

        let running_deployment = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status = 'RUNNING')",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?
            != 0;
        if running_deployment {
            return Err(sqlx::Error::Protocol(
                "compose deployment already running".into(),
            ));
        }

        let compose = sqlx::query_as!(
            ComposeRecord,
            r#"UPDATE compose_projects SET compose_status = ? WHERE id = ?
               RETURNING id AS "id!: i64", name, app_name, description, env_var, compose_file,
               source_type, compose_type, compose_status, trigger_type,
               repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
               gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
               bitbucket_branch, custom_git_url, custom_git_branch, command, compose_path,
               environment_id, server_id, created_at, updated_at"#,
            operation.target_status(),
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        let log_path = format!("logs/compose/{}/{}.log", id, Uuid::new_v4());
        let deployment = sqlx::query(
            r#"INSERT INTO deployments (title, description, status, state, log_path, compose_id, server_id, started_at, last_state_at)
               VALUES (?, ?, 'RUNNING', 'QUEUE', ?, ?, ?, strftime('%s', 'now'), strftime('%s', 'now'))
               RETURNING id"#,
        )
        .bind(operation.title())
        .bind(Some(format!("{} requested for {}", operation.as_str(), compose.name)))
        .bind(log_path)
        .bind(id)
        .bind(compose.server_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        let deployment_id: i64 = deployment.get("id");
        self.spawn_operation(id, deployment_id, operation);
        Ok(ComposeOperationResult {
            compose,
            deployment_id: Some(deployment_id),
            operation,
        })
    }

    pub async fn cancel_operation(&self, id: i64) -> sqlx::Result<bool> {
        self.get_by_id(id).await?;
        let has_running_deployment = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status = 'RUNNING')",
        )
        .bind(id)
        .fetch_one(self.db.as_ref())
        .await?
            != 0;
        if !has_running_deployment {
            return Ok(false);
        }

        let state = resolve::<ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        if !state.cancel_by_id(IdType::ComposeId(id)) {
            return Ok(false);
        }

        sqlx::query(
            "UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE compose_id = ? AND status = 'RUNNING'",
        )
        .bind(id)
        .execute(self.db.as_ref())
        .await?;
        Ok(true)
    }

    fn spawn_operation(&self, compose_id: i64, deployment_id: i64, operation: ComposeOperation) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let result = execute_operation(db.clone(), compose_id, deployment_id, operation).await;
            let (compose_status, deployment_status, error_message) = match result {
                Ok(()) => ("DONE", "DONE", None),
                Err(error) if is_cancelled_error(&error) => {
                    tracing::warn!(compose_id, deployment_id, operation = operation.as_str(), error = %error, "compose operation cancelled");
                    ("ERROR", "CANCELLED", Some(error))
                }
                Err(error) => {
                    tracing::error!(compose_id, deployment_id, operation = operation.as_str(), error = %error, "compose operation failed");
                    ("ERROR", "ERROR", Some(error))
                }
            };
            if let Err(error) =
                sqlx::query("UPDATE compose_projects SET compose_status = ? WHERE id = ?")
                    .bind(compose_status)
                    .bind(compose_id)
                    .execute(db.as_ref())
                    .await
            {
                tracing::error!(compose_id, error = %error, "could not persist compose status");
            }
            if let Err(error) = sqlx::query(
                "UPDATE deployments SET status = ?, state = ?, error_message = ?, finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE id = ?",
            )
            .bind(deployment_status)
            .bind(deployment_status)
            .bind(error_message)
            .bind(deployment_id)
            .execute(db.as_ref())
            .await
            {
                tracing::error!(deployment_id, error = %error, "could not persist compose deployment status");
            }
            if let Ok(state) = resolve::<ApplicationState>().await {
                state.remove_state(IdType::ComposeId(compose_id));
            }
        });
    }
}
