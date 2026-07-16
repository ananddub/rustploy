use auto_di::resolve;
use sqlx::Row;

use crate::utils::builder::queue::BuilderQueue;
use super::{DatabaseService, DatabaseKind, DatabaseOperation, DatabaseOperationResult};

impl DatabaseService {
    pub async fn run_operation(
        &self,
        kind: DatabaseKind,
        id: i64,
        operation: DatabaseOperation,
    ) -> sqlx::Result<DatabaseOperationResult> {
        let mut tx = self.db.begin().await?;

        let running_deployment = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM deployments WHERE database_id = ? AND status IN ('QUEUED', 'RUNNING'))",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?
            != 0;
        if running_deployment {
            return Err(sqlx::Error::Protocol(
                "database deployment already queued or running; cancel it first".into(),
            ));
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .ensure_capacity()
            .await?;

        let kind_str = kind.as_str();
        let table_name = match kind {
            DatabaseKind::Postgres => "postgres_dbs",
            DatabaseKind::Mysql => "mysql_dbs",
            DatabaseKind::Mariadb => "mariadb_dbs",
            DatabaseKind::Mongo => "mongo_dbs",
            DatabaseKind::Redis => "redis_dbs",
            DatabaseKind::Libsql => "libsql_dbs",
        };

        let (server_id, name) = match kind {
            DatabaseKind::Postgres => self.get_postgres_server_id_and_name(id).await?,
            DatabaseKind::Mysql => self.get_mysql_server_id_and_name(id).await?,
            DatabaseKind::Mariadb => self.get_mariadb_server_id_and_name(id).await?,
            DatabaseKind::Mongo => self.get_mongo_server_id_and_name(id).await?,
            DatabaseKind::Redis => self.get_redis_server_id_and_name(id).await?,
            DatabaseKind::Libsql => self.get_libsql_server_id_and_name(id).await?,
        };

        let update_status_query = format!("UPDATE {} SET app_status = ? WHERE id = ?", table_name);
        sqlx::query(sqlx::AssertSqlSafe(&*update_status_query))
            .bind("RUNNING")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        let log_path = format!("pending-db-{}", id);
        let deployment = sqlx::query(
            r#"INSERT INTO deployments (title, description, status, state, log_path, operation, database_id, database_kind, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, ?, strftime('%s', 'now'))
               RETURNING id"#,
        )
        .bind(operation.title())
        .bind(Some(format!("{} requested for database {}", operation.as_str(), name)))
        .bind(log_path)
        .bind(operation.as_str())
        .bind(id)
        .bind(kind_str)
        .bind(server_id)
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
            let _ = log.write_line(&format!("[QUEUED] database deployment queued for {}", operation.as_str())).await;
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .notify();

        Ok(DatabaseOperationResult {
            database: self.get_by_id(kind, id).await?,
            operation,
        })
    }
}
