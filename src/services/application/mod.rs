pub use types::{ApplicationOperation, ApplicationOperationResult, ApplicationRecord};

use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::repository::{ApplicationRepository, DeploymentRepository};

pub struct ApplicationService {
    pub(super) db: Arc<SqlitePool>,
    pub(super) repo_app: Arc<ApplicationRepository>,
    pub(super) repo_deploy: Arc<DeploymentRepository>,
}

#[singleton]
impl ApplicationService {
    fn new(
        db: Arc<SqlitePool>,
        repo_app: Arc<ApplicationRepository>,
        repo_deploy: Arc<DeploymentRepository>,
    ) -> Self {
        Self {
            db,
            repo_app,
            repo_deploy,
        }
    }
}

pub mod auto_excuter;
pub mod config;
pub mod crud;
pub mod operations;
pub mod queries;
pub mod remote;
pub mod rollback;
pub mod source;
pub mod types;
