use crate::config::init::Config;
use auto_di::singleton;
use bollard::Docker;
use std::sync::Arc;

pub struct AppState {
    pub config: Arc<Config>,
    pub db_pool: Arc<sqlx::SqlitePool>,
    pub docker: Arc<Docker>,
}

#[singleton]
impl AppState {
    pub fn new(config: Arc<Config>, db_pool: Arc<sqlx::SqlitePool>, docker: Arc<Docker>) -> Self {
        AppState {
            config,
            db_pool,
            docker,
        }
    }
}
