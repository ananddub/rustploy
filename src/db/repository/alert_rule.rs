use crate::db::models::alert_rule::AlertRule;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct AlertRuleRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl AlertRuleRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<AlertRule>, sqlx::Error> {
        sqlx::query_as!(
            AlertRule,
            r#"SELECT id AS "id?: i64", name, target_type, target_id, metric_name, operator, threshold AS "threshold: f64", duration_seconds AS "duration_seconds: i32", notification_channel, enabled AS "enabled: i32", created_at, updated_at FROM alert_rules ORDER BY created_at DESC"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<AlertRule>, sqlx::Error> {
        sqlx::query_as!(
            AlertRule,
            r#"SELECT id AS "id?: i64", name, target_type, target_id, metric_name, operator, threshold AS "threshold: f64", duration_seconds AS "duration_seconds: i32", notification_channel, enabled AS "enabled: i32", created_at, updated_at FROM alert_rules WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, rule: &AlertRule) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let res = sqlx::query!(
            r#"INSERT INTO alert_rules (name, target_type, target_id, metric_name, operator, threshold, duration_seconds, notification_channel, enabled, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rule.name,
            rule.target_type,
            rule.target_id,
            rule.metric_name,
            rule.operator,
            rule.threshold,
            rule.duration_seconds,
            rule.notification_channel,
            rule.enabled,
            now,
            now
        )
        .execute(self.pool.as_ref())
        .await?;

        Ok(res.last_insert_rowid())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM alert_rules WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
