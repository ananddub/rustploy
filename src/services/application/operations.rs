use auto_di::resolve;
use sqlx::Row;

use crate::services::application::auto_excuter::app_new_cmd;
use crate::utils::builder::queue::BuilderQueue;
use crate::utils::builder::{custom_type::IdType, hash_state::ApplicationState};
use crate::utils::docker::DockerCli;
use crate::utils::docker::query::filter::ServiceFilter;

use super::{
    ApplicationOperation, ApplicationOperationResult, ApplicationRecord, ApplicationService,
    // runtime::{execute_operation, is_cancelled_error},
};

impl ApplicationService {
    pub async fn run_operation(
        &self,
        id: i64,
        operation: ApplicationOperation,
    ) -> sqlx::Result<ApplicationOperationResult> {
        let mut tx = self.db.begin().await?;

        let running_deployment = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM deployments WHERE application_id = ? AND status IN ('QUEUED', 'RUNNING'))",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?
            != 0;
        if running_deployment {
            return Err(sqlx::Error::Protocol(
                "application deployment already queued or running; cancel it first".into(),
            ));
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .ensure_capacity()
            .await?;

        let app = sqlx::query_as!(
            ApplicationRecord,
            r#"UPDATE applications SET app_status = ? WHERE id = ?
               RETURNING id AS "id!: i64", name, app_name, description, source_type, build_type, app_status, trigger_type,
               environment_id, server_id, build_server_id, registry_id, env_var, icon,
               repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
               gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
               bitbucket_branch, docker_image, registry_url, custom_git_url, custom_git_branch,
               created_at, updated_at"#,
            operation.target_status(),
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        let log_path = format!("pending-app-{}", id);
        let deployment = sqlx::query(
            r#"INSERT INTO deployments (title, description, status, state, log_path, operation, application_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, strftime('%s', 'now'))
               RETURNING id"#,
        )
        .bind(operation.title())
        .bind(Some(format!("{} requested for {}", operation.as_str(), app.name)))
        .bind(log_path)
        .bind(operation.as_str())
        .bind(id)
        .bind(app.server_id)
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

        Ok(ApplicationOperationResult {
            application: app,
            deployment_id: Some(deployment_id),
            operation,
        })
    }

    pub async fn cancel_operation(&self, id: i64) -> sqlx::Result<bool> {
        let app_user = self.get_by_id(id).await?;

        let queue = resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        if queue.cancel_queued_application(id).await? {
            sqlx::query("UPDATE applications SET app_status = 'IDLE' WHERE id = ?")
                .bind(id)
                .execute(self.db.as_ref())
                .await?;
            return Ok(true);
        }

        let has_running_deployment = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM deployments WHERE application_id = ? AND status = 'RUNNING')",
        )
        .bind(id)
        .fetch_one(self.db.as_ref())
        .await?
            != 0;

        if !has_running_deployment {
            let cmd = app_new_cmd(self.db.clone(), id)
                .await
                .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
            let docker_cli = DockerCli::from_executor(cmd);
            let services = docker_cli
                .services()
                .list()
                .filter(ServiceFilter::name(format!("{}_", app_user.app_name)))
                .run_json()
                .await
                .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
            if services.is_empty() {
                return Ok(false);
            }
            let mut flag = false;
            for s in services.iter() {
                if &s.replicas != "0/0" {
                    flag = true;
                    docker_cli
                        .services()
                        .scale()
                        .service(&s.name, 0)
                        .run()
                        .await
                        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
                }
            }
            return Ok(flag);
        }

        let state = resolve::<ApplicationState>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        if !state.cancel_by_id(IdType::AppId(id)) {
            return Ok(false);
        }

        sqlx::query(
            "UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE application_id = ? AND status = 'RUNNING'",
        )
        .bind(id)
        .execute(self.db.as_ref())
        .await?;
        Ok(true)
    }

}
