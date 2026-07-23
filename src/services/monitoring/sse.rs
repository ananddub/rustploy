use auto_di::singleton;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContainerMetricSseEvent {
    pub server_id: i64,
    pub application_id: i64,
    pub compose_id: i64,
    pub container_id: String,
    pub container_name: String,
    pub cpu_percent: f64,
    pub memory_used_mb: f64,
    pub memory_limit_mb: f64,
    pub net_rx_kbps: f64,
    pub net_tx_kbps: f64,
    pub timestamp: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContainerLogSseEvent {
    pub container_id: String,
    pub log_line: String,
    pub timestamp: i64,
    pub is_stderr: bool,
}

pub struct MonitoringSseBus {
    metrics_tx: broadcast::Sender<ContainerMetricSseEvent>,
    logs_tx: broadcast::Sender<ContainerLogSseEvent>,
}

#[singleton]
impl MonitoringSseBus {
    pub fn new() -> Self {
        let (metrics_tx, _) = broadcast::channel(1000);
        let (logs_tx, _) = broadcast::channel(1000);
        Self {
            metrics_tx,
            logs_tx,
        }
    }

    pub fn publish_container_metric(&self, event: ContainerMetricSseEvent) {
        let _ = self.metrics_tx.send(event);
    }

    pub fn publish_log_chunk(&self, event: ContainerLogSseEvent) {
        let _ = self.logs_tx.send(event);
    }

    pub fn subscribe_container_metrics(&self) -> broadcast::Receiver<ContainerMetricSseEvent> {
        self.metrics_tx.subscribe()
    }

    pub fn subscribe_logs(&self) -> broadcast::Receiver<ContainerLogSseEvent> {
        self.logs_tx.subscribe()
    }
}
