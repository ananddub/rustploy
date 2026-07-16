use crate::db::models::rollbacks::Rollback;
use crate::db::models::types::*;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use std::sync::Arc;
use auto_di::singleton;

pub struct RollbackRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl RollbackRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Rollback>, sqlx::Error> {
        sqlx::query_as!(
            Rollback,
            r#"SELECT id AS "id?: i64", deployment_id AS "deployment_id: i64", version AS "version: i64", image AS "image?: String", full_context AS "full_context?: String", created_at AS "created_at: i64" FROM rollbacks"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Rollback>, sqlx::Error> {
        sqlx::query_as!(
            Rollback,
            r#"SELECT id AS "id?: i64", deployment_id AS "deployment_id: i64", version AS "version: i64", image AS "image?: String", full_context AS "full_context?: String", created_at AS "created_at: i64" FROM rollbacks WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Rollback) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO rollbacks (deployment_id, version, image, full_context, created_at) VALUES (?, ?, ?, ?, ?)"#,
            item.deployment_id,
            item.version,
            &item.image,
            &item.full_context,
            item.created_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Rollback) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE rollbacks SET deployment_id = ?, version = ?, image = ?, full_context = ?, created_at = ? WHERE id = ?"#,
            item.deployment_id,
            item.version,
            &item.image,
            &item.full_context,
            item.created_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM rollbacks WHERE id = ?"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
