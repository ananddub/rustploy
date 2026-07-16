use crate::db::models::deployments::Deployment;
use crate::db::models::types::*;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use std::sync::Arc;
use auto_di::singleton;

pub struct DeploymentRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl DeploymentRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Deployment>, sqlx::Error> {
        sqlx::query_as!(
            Deployment,
            r#"SELECT id AS "id?: i64", title AS "title: String", description AS "description?: String", status AS "status: String", state AS "state: String", log_path AS "log_path: String", pid AS "pid?: String", error_message AS "error_message?: String", operation AS "operation?: String", is_preview_deployment AS "is_preview_deployment: i64", started_at AS "started_at?: i64", last_state_at AS "last_state_at?: i64", finished_at AS "finished_at?: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", server_id AS "server_id?: i64", created_at AS "created_at: i64", database_id AS "database_id?: i64", database_kind AS "database_kind?: String" FROM deployments"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Deployment>, sqlx::Error> {
        sqlx::query_as!(
            Deployment,
            r#"SELECT id AS "id?: i64", title AS "title: String", description AS "description?: String", status AS "status: String", state AS "state: String", log_path AS "log_path: String", pid AS "pid?: String", error_message AS "error_message?: String", operation AS "operation?: String", is_preview_deployment AS "is_preview_deployment: i64", started_at AS "started_at?: i64", last_state_at AS "last_state_at?: i64", finished_at AS "finished_at?: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", server_id AS "server_id?: i64", created_at AS "created_at: i64", database_id AS "database_id?: i64", database_kind AS "database_kind?: String" FROM deployments WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Deployment) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, state, log_path, pid, error_message, operation, is_preview_deployment, started_at, last_state_at, finished_at, application_id, compose_id, server_id, created_at, database_id, database_kind) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.title,
            &item.description,
            &item.status,
            &item.state,
            &item.log_path,
            &item.pid,
            &item.error_message,
            &item.operation,
            item.is_preview_deployment,
            item.started_at,
            item.last_state_at,
            item.finished_at,
            item.application_id,
            item.compose_id,
            item.server_id,
            item.created_at,
            item.database_id,
            &item.database_kind
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Deployment) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE deployments SET title = ?, description = ?, status = ?, state = ?, log_path = ?, pid = ?, error_message = ?, operation = ?, is_preview_deployment = ?, started_at = ?, last_state_at = ?, finished_at = ?, application_id = ?, compose_id = ?, server_id = ?, created_at = ?, database_id = ?, database_kind = ? WHERE id = ?"#,
            &item.title,
            &item.description,
            &item.status,
            &item.state,
            &item.log_path,
            &item.pid,
            &item.error_message,
            &item.operation,
            item.is_preview_deployment,
            item.started_at,
            item.last_state_at,
            item.finished_at,
            item.application_id,
            item.compose_id,
            item.server_id,
            item.created_at,
            item.database_id,
            &item.database_kind,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM deployments WHERE id = ?"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn has_running_deployment(&self, application_id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE application_id = ? AND status IN ('QUEUED', 'RUNNING')) AS "exists_val!: i64""#,
            application_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn create_queued_deployment(
        &self,
        title: String,
        description: Option<String>,
        log_path: String,
        operation: String,
        application_id: i64,
        server_id: Option<i64>,
    ) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, state, log_path, operation, application_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, strftime('%s', 'now'))"#,
            title,
            description,
            log_path,
            operation,
            application_id,
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn update_log_path(&self, id: i64, log_path: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE deployments SET log_path = ? WHERE id = ?"#,
            log_path,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn has_running_status_deployment(&self, application_id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE application_id = ? AND status = 'RUNNING') AS "exists_val!: i64""#,
            application_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn request_cancel_deployment(&self, application_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE application_id = ? AND status = 'RUNNING'"#,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn has_running_compose_deployment(&self, compose_id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status IN ('QUEUED', 'RUNNING')) AS "exists_val!: i64""#,
            compose_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn create_queued_compose_deployment(
        &self,
        title: String,
        description: Option<String>,
        log_path: String,
        operation: String,
        compose_id: i64,
        server_id: Option<i64>,
    ) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, state, log_path, operation, compose_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, strftime('%s', 'now'))"#,
            title,
            description,
            log_path,
            operation,
            compose_id,
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn has_running_status_compose_deployment(&self, compose_id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status = 'RUNNING') AS "exists_val!: i64""#,
            compose_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn request_cancel_compose_deployment(&self, compose_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE compose_id = ? AND status = 'RUNNING'"#,
            compose_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn has_running_database_deployment(&self, database_id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE database_id = ? AND status IN ('QUEUED', 'RUNNING')) AS "exists_val!: i64""#,
            database_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn create_queued_database_deployment(
        &self,
        title: String,
        description: Option<String>,
        log_path: String,
        operation: String,
        database_id: i64,
        database_kind: String,
        server_id: Option<i64>,
    ) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, state, log_path, operation, database_id, database_kind, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, ?, strftime('%s', 'now'))"#,
            title,
            description,
            log_path,
            operation,
            database_id,
            database_kind,
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn get_cancel_info(&self, id: i64) -> Result<Option<(Option<i64>, Option<i64>, Option<i64>, String)>, sqlx::Error> {
        let res = sqlx::query!(
            r#"SELECT application_id, compose_id, database_id, status FROM deployments WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(res.map(|r| (r.application_id, r.compose_id, r.database_id, r.status)))
    }

    pub async fn cancel_queued_deployment(&self, id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE id = ? AND status = 'QUEUED'"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn set_cancel_requested(&self, id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE id = ? AND status = 'RUNNING'"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn list_filtered(
        &self,
        status: Option<String>,
        state: Option<String>,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        database_id: Option<i64>,
        server_id: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Deployment>, sqlx::Error> {
        sqlx::query_as!(
            Deployment,
            r#"SELECT id AS "id?: i64", title AS "title: String", description AS "description?: String", status AS "status: String", state AS "state: String", log_path AS "log_path: String", pid AS "pid?: String", error_message AS "error_message?: String", operation AS "operation?: String", is_preview_deployment AS "is_preview_deployment: i64", started_at AS "started_at?: i64", last_state_at AS "last_state_at?: i64", finished_at AS "finished_at?: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", server_id AS "server_id?: i64", created_at AS "created_at: i64", database_id AS "database_id?: i64", database_kind AS "database_kind?: String"
               FROM deployments
               WHERE (? IS NULL OR status = ?)
                 AND (? IS NULL OR state = ?)
                 AND (? IS NULL OR application_id = ?)
                 AND (? IS NULL OR compose_id = ?)
                 AND (? IS NULL OR database_id = ?)
                 AND (? IS NULL OR server_id = ?)
               ORDER BY COALESCE(started_at, created_at) DESC, id DESC
               LIMIT ? OFFSET ?"#,
            status,
            status,
            state,
            state,
            application_id,
            application_id,
            compose_id,
            compose_id,
            database_id,
            database_id,
            server_id,
            server_id,
            limit,
            offset,
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_status(&self, id: i64) -> Result<Option<String>, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT status FROM deployments WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(res)
    }

    pub async fn get_latest_application_deployment_id(&self, application_id: i64) -> Result<Option<i64>, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT id AS "id: i64" FROM deployments WHERE application_id = ? ORDER BY id DESC LIMIT 1"#,
            application_id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(res)
    }

    pub async fn get_latest_compose_deployment_id(&self, compose_id: i64) -> Result<Option<i64>, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT id AS "id: i64" FROM deployments WHERE compose_id = ? ORDER BY id DESC LIMIT 1"#,
            compose_id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(res)
    }
}
