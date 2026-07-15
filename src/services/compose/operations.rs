use auto_di::resolve;
use sqlx::Row;
use std::sync::Arc;
use sqlx::SqlitePool;

use crate::utils::builder::queue::BuilderQueue;
use crate::utils::builder::{custom_type::IdType, hash_state::ApplicationState};
use crate::utils::docker::DockerCli;
use crate::utils::docker::query::ServiceFilter;
use super::{ComposeOperation, ComposeOperationResult, ComposeRecord, ComposeService, auto_excuter::compose_new_db, ComposeType};

impl ComposeService {
    pub async fn run_operation(
        &self,
        id: i64,
        operation: ComposeOperation,
    ) -> sqlx::Result<ComposeOperationResult> {
        let mut tx = self.db.begin().await?;

        let running_deployment = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status IN ('QUEUED', 'RUNNING'))",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?
            != 0;
        if running_deployment {
            return Err(sqlx::Error::Protocol(
                "compose deployment already queued or running".into(),
            ));
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .ensure_capacity()
            .await?;

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

        let log_path = format!("pending-compose-{}", id);
        let deployment = sqlx::query(
            r#"INSERT INTO deployments (title, description, status, state, log_path, operation, compose_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, strftime('%s', 'now'))
               RETURNING id"#,
        )
        .bind(operation.title())
        .bind(Some(format!("{} requested for {}", operation.as_str(), compose.name)))
        .bind(log_path)
        .bind(operation.as_str())
        .bind(id)
        .bind(compose.server_id)
        .fetch_one(&mut *tx)
        .await?;

        let deployment_id: i64 = deployment.get("id");
        let log_path = crate::utils::paths::rustploy_paths().deployment_log_file(deployment_id);

        sqlx::query("UPDATE deployments SET log_path = ? WHERE id = ?")
            .bind(&log_path)
            .bind(deployment_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        if let Ok(mut log) = crate::utils::builder::queue::deployment_log::DeploymentLog::open(deployment_id).await {
            let _ = log.write_line(&format!("[QUEUED] deployment queued for {}", operation.as_str())).await;
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .notify();

        Ok(ComposeOperationResult {
            compose,
            deployment_id: Some(deployment_id),
            operation,
        })
    }

    pub async fn cancel_operation(&self, id: i64) -> sqlx::Result<bool> {
        let compose = self.get_by_id(id).await?;

        let queue = resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        if queue.cancel_queued_compose(id).await? {
            sqlx::query("UPDATE compose_projects SET compose_status = 'IDLE' WHERE id = ?")
                .bind(id)
                .execute(self.db.as_ref())
                .await?;
            return Ok(true);
        }

        let has_running_deployment = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status = 'RUNNING')",
        )
        .bind(id)
        .fetch_one(self.db.as_ref())
        .await?
            != 0;

        if !has_running_deployment {

            let db = self.db.clone();
            let app_name = compose.app_name.clone();
            let compose_type = compose.compose_type.clone();
            tokio::spawn(async move {
                if let Err(e) = scale_down_compose(db.clone(), id, &app_name, compose_type).await {
                    tracing::warn!(compose_id = id, error = %e, "could not scale down compose on cancel");
                }
                let _ = sqlx::query("UPDATE compose_projects SET compose_status = 'IDLE' WHERE id = ?")
                    .bind(id)
                    .execute(db.as_ref())
                    .await.map_err(|e| tracing::error!(compose_id = id, error = %e, "could not update compose status to IDLE on cancel"));
            });
            return Ok(true);
        }

        // Case 2: RUNNING — signal cancellation token (stops build phase).
        let state = resolve::<ApplicationState>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        state.cancel_by_id(IdType::ComposeId(id));

        // Immediately mark status IDLE so new deployments are not blocked.
        sqlx::query("UPDATE compose_projects SET compose_status = 'IDLE' WHERE id = ?")
            .bind(id)
            .execute(self.db.as_ref())
            .await?;

        sqlx::query(
            "UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE compose_id = ? AND status = 'RUNNING'",
        )
        .bind(id)
        .execute(self.db.as_ref())
        .await?;

        // Case 3: also scale down Docker — build may have already deployed.
        let db = self.db.clone();
        let app_name = compose.app_name.clone();
        let compose_type = compose.compose_type.clone();
        tokio::spawn(async move {
            if let Err(e) = scale_down_compose(db, id, &app_name, compose_type).await {
                tracing::warn!(compose_id = id, error = %e, "could not scale down compose on cancel");
            }
        });

        Ok(true)
    }


}

async fn scale_down_compose(
    db: Arc<SqlitePool>,
    compose_id: i64,
    app_name: &str,
    compose_type: ComposeType,
) -> Result<(), String> {
    let cmd = compose_new_db(db, compose_id)
        .await
        .map_err(|e| format!("could not build executor: {e}"))?;
    let docker = DockerCli::from_executor(cmd);

    match compose_type {
        ComposeType::DockerCompose => {
            docker.compose().down().project(app_name).run().await
                .map(|_| ())
                .map_err(|e| format!("compose down failed: {e}"))
        }
        ComposeType::Stack => {
            let services = docker.services().list().filter(ServiceFilter::Name(app_name.to_string())).run_json()
                .await
                .map_err(|e| format!("could not get stack service: {e}"))?;

            if services.is_empty() {
                return Ok(());
            }
            for service in services {
                if service.replicas != "0/0" {
                    docker.services().scale().service(&service.name,0).run().await.map_err(|e| format!("service scale 0 failed: {e}"))?;
                }
            }
            Ok(())

        }
    }
}
