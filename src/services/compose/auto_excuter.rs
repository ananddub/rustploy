use std::sync::Arc;
use sqlx::SqlitePool;
use crate::utils::exec::{CommandExecutor, LocalExecutor, RemoteExecutor};

pub async  fn compose_new_db(db :Arc<SqlitePool>, compose_id: i64) -> Result<CommandExecutor, sqlx::Error> {
    let compose_user = sqlx::query!(
        "SELECT * FROM compose_projects WHERE id = ?",
        compose_id
    )
        .fetch_one(db.as_ref())
        .await?;
    let mut cmd:CommandExecutor ;
    if compose_user.server_id.is_none() {
        cmd  = CommandExecutor::Local(LocalExecutor::new());
        tracing::warn!(application_id = compose_user.id, "application has no server assigned; cannot cancel operation");
    }else {
        let rm = RemoteExecutor::new_with_db(db.clone(), compose_user.server_id.unwrap_or(0)).await?;
        cmd = CommandExecutor::Remote(rm)
    }
    Ok(cmd)
}