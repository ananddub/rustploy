pub use types::{ComposeOperation, ComposeOperationResult, ComposeRecord, ComposeType};

use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

pub struct ComposeService {
    pub(super) db: Arc<SqlitePool>,
}

#[singleton]
impl ComposeService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }
}

pub mod auto_excuter;
pub mod crud;
pub mod operations;
pub mod queries;
pub mod recovery;
pub mod remote;
pub mod source;
pub mod types;
