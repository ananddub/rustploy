use crate::core::config::Config;
use auto_di::singleton;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use std::sync::Arc;

#[singleton]
pub async fn connect(config: Arc<Config>) -> SqlitePool {
    let options = SqliteConnectOptions::from_str(config.database_url.as_str())
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect_with(options)
        .await
        .expect("Failed to connect to SQLite database");

    sqlx::migrate!("./db/migrations/")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    tracing::info!("Database connection established and migrations run successfully.");
    pool
}
