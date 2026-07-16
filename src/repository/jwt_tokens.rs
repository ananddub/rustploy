use crate::db::models::jwt_tokens::JwtToken;
use crate::db::models::types::*;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use std::sync::Arc;
use auto_di::singleton;

pub struct JwtTokenRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl JwtTokenRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<JwtToken>, sqlx::Error> {
        sqlx::query_as!(
            JwtToken,
            r#"SELECT id AS "id?: i64", jti AS "jti: String", role AS "role: String", user_id AS "user_id: i64", is_blacklist AS "is_blacklist?: i64", blacklist_at AS "blacklist_at?: i64", expired_at AS "expired_at?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM jwt_tokens"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<JwtToken>, sqlx::Error> {
        sqlx::query_as!(
            JwtToken,
            r#"SELECT id AS "id?: i64", jti AS "jti: String", role AS "role: String", user_id AS "user_id: i64", is_blacklist AS "is_blacklist?: i64", blacklist_at AS "blacklist_at?: i64", expired_at AS "expired_at?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM jwt_tokens WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &JwtToken) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO jwt_tokens (jti, role, user_id, is_blacklist, blacklist_at, expired_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.jti,
            &item.role,
            item.user_id,
            item.is_blacklist,
            item.blacklist_at,
            item.expired_at,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &JwtToken) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE jwt_tokens SET jti = ?, role = ?, user_id = ?, is_blacklist = ?, blacklist_at = ?, expired_at = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.jti,
            &item.role,
            item.user_id,
            item.is_blacklist,
            item.blacklist_at,
            item.expired_at,
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
            r#"DELETE FROM jwt_tokens WHERE id = ?"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
