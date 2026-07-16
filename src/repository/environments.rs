use crate::db::models::environments::Environment;
use crate::db::models::types::*;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use std::sync::Arc;
use auto_di::singleton;

pub struct EnvironmentRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl EnvironmentRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Environment>, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", env_var AS "env_var: String", is_default AS "is_default: i64", project_id AS "project_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM environments"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Environment>, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", env_var AS "env_var: String", is_default AS "is_default: i64", project_id AS "project_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM environments WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Environment) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO environments (name, description, env_var, is_default, project_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.description,
            &item.env_var,
            item.is_default,
            item.project_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Environment) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE environments SET name = ?, description = ?, env_var = ?, is_default = ?, project_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.description,
            &item.env_var,
            item.is_default,
            item.project_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM environments WHERE id = ?"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
