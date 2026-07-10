use crate::core::config::Config;
use auto_di::singleton;
use std::sync::Arc;

pub struct AppState {
    pub config: Arc<Config>,
    pub db_pool: Arc<sqlx::SqlitePool>,
}

#[singleton]
impl AppState {
    pub fn new(config: Arc<Config>, db_pool: Arc<sqlx::SqlitePool>) -> Self {
        AppState { config, db_pool }
    }
}
