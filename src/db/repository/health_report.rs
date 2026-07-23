use crate::db::models::health_report::HealthReport;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct HealthReportRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl HealthReportRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn list(&self, limit: i64) -> Result<Vec<HealthReport>, sqlx::Error> {
        sqlx::query_as!(
            HealthReport,
            r#"SELECT id AS "id?: i64", target_id, target_type, status, response_time_ms AS "response_time_ms: i32", error_message, created_at FROM health_reports ORDER BY created_at DESC LIMIT ?"#,
            limit
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, report: &HealthReport) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let res = sqlx::query!(
            r#"INSERT INTO health_reports (target_id, target_type, status, response_time_ms, error_message, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
            report.target_id,
            report.target_type,
            report.status,
            report.response_time_ms,
            report.error_message,
            now
        )
        .execute(self.pool.as_ref())
        .await?;

        Ok(res.last_insert_rowid())
    }
}
