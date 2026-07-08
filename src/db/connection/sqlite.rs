use crate::core::config::Config;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

#[singleton]
pub async fn connect(config: Arc<Config>) -> SqlitePool {
    let pool = SqlitePool::connect(config.database_url.as_str())
        .await
        .expect("Failed to connect to SQLite database");

    sqlx::migrate!("./db/migrations/")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    tracing::info!("Database connection established and migrations run successfully.");
    pool
}
