
pub use types::{ComposeOperation, ComposeOperationResult, ComposeRecord};

use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

pub struct ComposeService {
    pub(super) db: Arc<SqlitePool>,
}

#[singleton]
impl ComposeService {
    fn new(db: Arc<SqlitePool>) -> Self {
        recovery::spawn_recover_stale_deployments(db.clone());
        Self { db }
    }
}

pub mod crud;
pub mod operations;
pub mod queries;
pub mod recovery;
pub mod remote;
pub mod runtime;
pub mod source;
pub mod types;
