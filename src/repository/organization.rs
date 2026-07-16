use crate::db::models::organization::Organization;
use crate::db::models::types::*;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use std::sync::Arc;
use auto_di::singleton;

pub struct OrganizationRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl OrganizationRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Organization>, sqlx::Error> {
        sqlx::query_as!(
            Organization,
            r#"SELECT id AS "id?: i64", name AS "name: String", logo AS "logo?: String", slug AS "slug: String", owner_id AS "owner_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM organization"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Organization>, sqlx::Error> {
        sqlx::query_as!(
            Organization,
            r#"SELECT id AS "id?: i64", name AS "name: String", logo AS "logo?: String", slug AS "slug: String", owner_id AS "owner_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM organization WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Organization) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO organization (name, logo, slug, owner_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.logo,
            &item.slug,
            item.owner_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Organization) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE organization SET name = ?, logo = ?, slug = ?, owner_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.logo,
            &item.slug,
            item.owner_id,
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
            r#"DELETE FROM organization WHERE id = ?"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
