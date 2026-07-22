use auto_di::singleton;
use std::sync::Arc;

use crate::db::models::server_metrics::ServerMetric;
use crate::repository::server_metrics::ServerMetricRepository;

pub struct MonitoringService {
    metrics_repo: Arc<ServerMetricRepository>,
}

#[singleton]
impl MonitoringService {
    pub fn new(metrics_repo: Arc<ServerMetricRepository>) -> Self {
        Self { metrics_repo }
    }

    pub async fn record_server_metric(&self, metric: ServerMetric) -> sqlx::Result<()> {
        self.metrics_repo.create(&metric).await?;
        Ok(())
    }

    pub async fn get_latest_metrics(&self, limit: i64) -> sqlx::Result<Vec<ServerMetric>> {
        self.metrics_repo.get_latest(limit).await
    }
}
