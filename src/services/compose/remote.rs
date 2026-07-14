use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use sqlx::SqlitePool;

use crate::utils::{
    exec::{RemoteExecutor, SshAuth, SshHostKey},
    session::RemoteExecutorRegistry,
};

pub(crate) fn deployment_pid_file(deployment_id: i64) -> String {
    format!("/tmp/rustploy-deployment-{deployment_id}.pid")
}

pub(crate) async fn remote_executor(
    db: &SqlitePool,
    server_id: i64,
) -> Result<RemoteExecutor, String> {
    let row = sqlx::query_as::<_, (String, i64, String, String, String)>(
        r#"SELECT s.ip_address, s.port, s.username, k.private_key, k.public_key
           FROM servers s JOIN ssh_keys k ON k.id = s.ssh_key_id WHERE s.id = ?"#,
    )
    .bind(server_id)
    .fetch_one(db)
    .await
    .map_err(|error| format!("could not load SSH credentials: {error}"))?;
    let mut hasher = DefaultHasher::new();
    row.hash(&mut hasher);
    let version = hasher.finish();
    if let Some(executor) = RemoteExecutorRegistry::global().get(server_id, version) {
        return Ok(executor);
    }
    let port = u16::try_from(row.1).map_err(|_| "SSH port must be between 0 and 65535")?;
    tracing::warn!(
        server_id,
        "compose deployment SSH host key verification is disabled because no fingerprint is stored for this server"
    );
    let executor = RemoteExecutor::new(
        row.0,
        port,
        row.2,
        SshAuth::key_pair(row.3, row.4),
        SshHostKey::InsecureAcceptAny,
    )
    .with_pool_size(4)
    .with_sudo();
    RemoteExecutorRegistry::global().insert(server_id, version, executor.clone());
    Ok(executor)
}
