use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::{
    api::dto::ssh_key::{CreateSshKeyDto, PatchSshKeyDto},
    db::models::ssh_keys::SshKey,
};

pub struct SshKeyService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl SshKeyService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<SshKey> {
        sqlx::query_as!(
            SshKey,
            r#"SELECT id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at
               FROM ssh_keys WHERE id = ?"#,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn list(&self) -> sqlx::Result<Vec<SshKey>> {
        sqlx::query_as!(
            SshKey,
            r#"SELECT id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at
               FROM ssh_keys ORDER BY created_at DESC, id DESC"#
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn create(&self, input: CreateSshKeyDto) -> sqlx::Result<SshKey> {
        sqlx::query_as!(
            SshKey,
            r#"INSERT INTO ssh_keys (name, description, private_key, public_key)
               VALUES (?, ?, ?, ?)
               RETURNING id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at"#,
            input.name,
            input.description,
            input.private_key,
            input.public_key
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn patch(&self, id: i64, input: PatchSshKeyDto) -> sqlx::Result<SshKey> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let private_key = input.private_key.unwrap_or(current.private_key);
        let public_key = input.public_key.unwrap_or(current.public_key);

        sqlx::query_as!(
            SshKey,
            r#"UPDATE ssh_keys SET name = ?, description = ?, private_key = ?, public_key = ?
               WHERE id = ?
               RETURNING id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at"#,
            name,
            description,
            private_key,
            public_key,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn mark_used(&self, id: i64) -> sqlx::Result<SshKey> {
        sqlx::query_as!(
            SshKey,
            r#"UPDATE ssh_keys SET last_used_at = strftime('%s', 'now')
               WHERE id = ?
               RETURNING id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at"#,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM ssh_keys WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}
