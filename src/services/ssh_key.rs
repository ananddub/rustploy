use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::{
    api::dto::ssh_key::{CreateSshKeyDto, PatchSshKeyDto},
    db::models::ssh_keys::SshKey,
    repository::SshKeyRepository,
};

pub struct SshKeyService {
    db: Arc<SqlitePool>,
    repo_ssh: Arc<SshKeyRepository>,
}

#[singleton]
impl SshKeyService {
    fn new(db: Arc<SqlitePool>, repo_ssh: Arc<SshKeyRepository>) -> Self {
        Self { db, repo_ssh }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<SshKey> {
        self.repo_ssh
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list(&self) -> sqlx::Result<Vec<SshKey>> {
        self.repo_ssh.list_ordered().await
    }

    pub async fn create(&self, input: CreateSshKeyDto) -> sqlx::Result<SshKey> {
        self.repo_ssh.create_and_return(
            input.name,
            input.description,
            input.private_key,
            input.public_key
        ).await
    }

    pub async fn patch(&self, id: i64, input: PatchSshKeyDto) -> sqlx::Result<SshKey> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let private_key = input.private_key.unwrap_or(current.private_key);
        let public_key = input.public_key.unwrap_or(current.public_key);

        self.repo_ssh.update_and_return(id, name, description, private_key, public_key).await
    }

    pub async fn mark_used(&self, id: i64) -> sqlx::Result<SshKey> {
        self.repo_ssh.touch_and_return(id).await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        self.repo_ssh.delete(id).await
    }
}
