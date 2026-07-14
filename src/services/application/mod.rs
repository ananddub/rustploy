pub use types::{ApplicationOperation, ApplicationOperationResult, ApplicationRecord};

use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

pub struct ApplicationService {
    pub(super) db: Arc<SqlitePool>,
}

#[singleton]
impl ApplicationService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }
}

pub mod auto_excuter;
pub mod config;
pub mod crud;
pub mod operations;
pub mod queries;
pub mod recovery;
pub mod remote;
pub mod source;
pub mod types;
