use crate::db::{models::alert_rule::AlertRule, repository::AlertRuleRepository};
use auto_di::singleton;
use std::sync::Arc;

pub struct AlertService {
    repo: Arc<AlertRuleRepository>,
}

#[singleton]
impl AlertService {
    pub fn new(repo: Arc<AlertRuleRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_rules(&self) -> Result<Vec<AlertRule>, String> {
        self.repo.list().await.map_err(|e| e.to_string())
    }

    pub async fn create_rule(&self, rule: AlertRule) -> Result<i64, String> {
        self.repo.create(&rule).await.map_err(|e| e.to_string())
    }

    pub async fn delete_rule(&self, id: i64) -> Result<(), String> {
        self.repo.delete(id).await.map_err(|e| e.to_string())
    }
}
