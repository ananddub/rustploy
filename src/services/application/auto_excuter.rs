use std::sync::Arc;
use sqlx::SqlitePool;
use crate::utils::exec::{CommandExecutor, LocalExecutor, RemoteExecutor};

pub async  fn app_new_cmd(db :Arc<SqlitePool>, app_id: i64) -> Result<CommandExecutor, sqlx::Error> {
    let app_user = sqlx::query!(
        "SELECT * FROM applications WHERE id = ?",
        app_id
    )
    .fetch_one(db.as_ref())
    .await?;
    let mut cmd:CommandExecutor ;
    if app_user.server_id.is_none() {
        cmd  = CommandExecutor::Local(LocalExecutor::new());
        tracing::warn!(application_id = app_user.id, "application has no server assigned; cannot cancel operation");
    }else {
        let rm = RemoteExecutor::new_with_db(db.clone(), app_user.server_id.unwrap_or(0)).await?;
        cmd = CommandExecutor::Remote(rm)
    }
    Ok(cmd)
}