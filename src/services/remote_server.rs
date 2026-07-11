use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    api::dto::remote_server::{CreateRemoteServerDto, PatchRemoteServerDto},
    db::models::servers::Server,
};

pub struct ServerService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl ServerService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Server> {
        sqlx::query_as!(
            Server,
            r#"SELECT id AS "id?", name, description, ip_address, port, username, app_name,
               server_status, server_type, enable_docker_cleanup, log_cleanup_cron, command,
               metrics_config, ssh_key_id, created_at, updated_at
               FROM servers WHERE id = ?"#,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn list(&self) -> sqlx::Result<Vec<Server>> {
        sqlx::query_as!(
            Server,
            r#"SELECT id AS "id?", name, description, ip_address, port, username, app_name,
               server_status, server_type, enable_docker_cleanup, log_cleanup_cron, command,
               metrics_config, ssh_key_id, created_at, updated_at
               FROM servers ORDER BY created_at DESC, id DESC"#
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn connection_details(
        &self,
        id: i64,
    ) -> sqlx::Result<(Server, crate::db::models::ssh_keys::SshKey)> {
        let server = self.get_by_id(id).await?;
        let key_id = server.ssh_key_id.ok_or(sqlx::Error::RowNotFound)?;
        let key=sqlx::query_as!(crate::db::models::ssh_keys::SshKey,r#"SELECT id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at FROM ssh_keys WHERE id = ?"#,key_id).fetch_one(self.db.as_ref()).await?;
        Ok((server, key))
    }

    pub async fn create(&self, input: CreateRemoteServerDto) -> sqlx::Result<Server> {
        let app_name = generate_app_name(&input.name);

        sqlx::query_as!(
            Server,
            r#"INSERT INTO servers
               (name, description, ip_address, port, username, app_name, server_type, ssh_key_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS "id?", name, description, ip_address, port, username, app_name,
               server_status, server_type, enable_docker_cleanup, log_cleanup_cron, command,
               metrics_config, ssh_key_id, created_at, updated_at"#,
            input.name,
            input.description,
            input.ip_address,
            input.port,
            input.username,
            app_name,
            input.server_type,
            input.ssh_key_id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn patch(&self, id: i64, input: PatchRemoteServerDto) -> sqlx::Result<Server> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let ip_address = input.ip_address.unwrap_or(current.ip_address);
        let port = input.port.unwrap_or(current.port);
        let username = input.username.unwrap_or(current.username);
        let server_status = input.server_status.unwrap_or(current.server_status);
        let server_type = input.server_type.unwrap_or(current.server_type);
        let enable_docker_cleanup = input
            .enable_docker_cleanup
            .unwrap_or(current.enable_docker_cleanup);
        let log_cleanup_cron = input.log_cleanup_cron.or(current.log_cleanup_cron);
        let command = input.command.unwrap_or(current.command);
        let metrics_config = input.metrics_config.unwrap_or(current.metrics_config);
        let ssh_key_id = input.ssh_key_id.or(current.ssh_key_id);

        sqlx::query_as!(
            Server,
            r#"UPDATE servers SET
               name = ?, description = ?, ip_address = ?, port = ?, username = ?,
               server_status = ?, server_type = ?, enable_docker_cleanup = ?,
               log_cleanup_cron = ?, command = ?, metrics_config = ?, ssh_key_id = ?
               WHERE id = ?
               RETURNING id AS "id?", name, description, ip_address, port, username, app_name,
               server_status, server_type, enable_docker_cleanup, log_cleanup_cron, command,
               metrics_config, ssh_key_id, created_at, updated_at"#,
            name,
            description,
            ip_address,
            port,
            username,
            server_status,
            server_type,
            enable_docker_cleanup,
            log_cleanup_cron,
            command,
            metrics_config,
            ssh_key_id,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn set_status(&self, id: i64, status: &str) -> sqlx::Result<Server> {
        sqlx::query_as!(
            Server,
            r#"UPDATE servers SET server_status = ? WHERE id = ?
               RETURNING id AS "id?", name, description, ip_address, port, username, app_name,
               server_status, server_type, enable_docker_cleanup, log_cleanup_cron, command,
               metrics_config, ssh_key_id, created_at, updated_at"#,
            status,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn touch_test_connection(&self, id: i64) -> sqlx::Result<Server> {
        let server = self.get_by_id(id).await?;
        if let Some(ssh_key_id) = server.ssh_key_id {
            sqlx::query!(
                "UPDATE ssh_keys SET last_used_at = strftime('%s', 'now') WHERE id = ?",
                ssh_key_id
            )
            .execute(self.db.as_ref())
            .await?;
        }
        Ok(server)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM servers WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}

fn generate_app_name(name: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;

    for ch in name.to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            previous_dash = false;
        } else if !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }

    let slug = slug.trim_matches('-');
    let base = if slug.is_empty() { "server" } else { slug };
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}", base, &suffix[..6])
}
