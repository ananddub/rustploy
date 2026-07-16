pub mod postgres;
pub mod mysql;
pub mod mariadb;
pub mod mongo;
pub mod redis;
pub mod libsql;

pub mod types;
pub mod queries;
pub mod crud;
pub mod operations;

pub use types::{DatabaseKind, DatabaseRecord, DatabaseOperation, DatabaseOperationResult};

use std::sync::Arc;
use auto_di::singleton;
use sqlx::SqlitePool;

pub struct DatabaseService {
    pub(super) db: Arc<SqlitePool>,
}

#[singleton]
impl DatabaseService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }
}
