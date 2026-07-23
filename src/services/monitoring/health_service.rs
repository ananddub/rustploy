use crate::db::{models::health_report::HealthReport, repository::HealthReportRepository};
use auto_di::singleton;
use std::sync::Arc;

pub struct HealthService {
    repo: Arc<HealthReportRepository>,
}

#[singleton]
impl HealthService {
    pub fn new(repo: Arc<HealthReportRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_reports(&self, limit: i64) -> Result<Vec<HealthReport>, String> {
        self.repo.list(limit).await.map_err(|e| e.to_string())
    }

    pub async fn record_report(&self, report: HealthReport) -> Result<i64, String> {
        self.repo.create(&report).await.map_err(|e| e.to_string())
    }
}
