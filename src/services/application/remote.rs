use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use sqlx::SqlitePool;

use crate::utils::{
    exec::{RemoteExecutor, SshAuth, SshHostKey},
    session::RemoteExecutorRegistry,
};
use crate::repository::ServerRepository;
use auto_di::resolve;

pub(crate) async fn remote_executor(
    _db: &SqlitePool,
    server_id: i64,
) -> Result<RemoteExecutor, String> {
    let repo_servers = resolve::<ServerRepository>().await.unwrap();
    let row = repo_servers
        .get_ssh_credentials(server_id)
        .await
        .map_err(|error| format!("could not load SSH credentials: {error}"))?
        .ok_or_else(|| "Server not found".to_string())?;

    let mut hasher = DefaultHasher::new();
    row.hash(&mut hasher);
    let version = hasher.finish();
    if let Some(executor) = RemoteExecutorRegistry::global().get(server_id, version) {
        return Ok(executor);
    }
    let port = u16::try_from(row.1).map_err(|_| "SSH port must be between 0 and 65535")?;
    tracing::warn!(
        server_id,
        "deployment SSH host key verification is disabled because no fingerprint is stored for this server"
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
