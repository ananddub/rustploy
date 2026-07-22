use crate::docker::DockerMonitor;
use crate::grpc_client::{GrpcTelemetryPusher, SystemMetricsRequest};
use reqwest::Client;
use serde::Serialize;
use sqlx::SqlitePool;
use std::sync::Mutex as StdMutex;
use tokio::sync::Mutex as AsyncMutex;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{Disks, Networks, System};
use tracing::{error, info};

#[derive(Debug, Serialize)]
pub struct TelemetryPayload {
    pub server_id: i64,
    pub cpu_usage: f64,
    pub memory_used_mb: f64,
    pub memory_total_mb: f64,
    pub memory_percent: f64,
    pub disk_used_gb: f64,
    pub disk_total_gb: f64,
    pub disk_percent: f64,
    pub net_rx_kbps: f64,
    pub net_tx_kbps: f64,
    pub timestamp: i64,
}

pub struct CollectorService {
    pool: SqlitePool,
    sys: StdMutex<System>,
    http_client: Client,
    docker: DockerMonitor,
    grpc_pusher: AsyncMutex<Option<GrpcTelemetryPusher>>,
}

impl CollectorService {
    pub fn new(pool: SqlitePool, grpc_pusher: Option<GrpcTelemetryPusher>) -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            pool,
            sys: StdMutex::new(sys),
            http_client: Client::new(),
            docker: DockerMonitor::new(),
            grpc_pusher: AsyncMutex::new(grpc_pusher),
        }
    }

    pub async fn start_background_collector(self: std::sync::Arc<Self>, interval_secs: u64) {
        info!("Starting Hardware & Docker gRPC Telemetry Collector (Interval: {}s)", interval_secs);
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));

        loop {
            interval.tick().await;
            if let Err(err) = self.collect_and_push_metrics().await {
                error!("Error collecting/pushing telemetry: {:?}", err);
            }
        }
    }

    async fn collect_and_push_metrics(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        // 1. Sample System CPU & Memory
        let (cpu_usage, memory_used_mb, memory_total_mb, memory_percent) = {
            let mut sys = self.sys.lock().map_err(|e| e.to_string())?;
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let cpu = sys.global_cpu_usage() as f64;
            let mem_total_mb = sys.total_memory() as f64 / 1024.0 / 1024.0;
            let mem_used_mb = sys.used_memory() as f64 / 1024.0 / 1024.0;
            let mem_percent = if mem_total_mb > 0.0 {
                (mem_used_mb / mem_total_mb) * 100.0
            } else {
                0.0
            };
            (cpu, mem_used_mb, mem_total_mb, mem_percent)
        };

        // 2. Sample Disk Usage
        let disks = Disks::new_with_refreshed_list();
        let mut disk_used_bytes = 0u64;
        let mut disk_total_bytes = 0u64;
        for disk in &disks {
            let total = disk.total_space();
            let avail = disk.available_space();
            disk_total_bytes += total;
            disk_used_bytes += total.saturating_sub(avail);
        }
        let disk_used_gb = disk_used_bytes as f64 / 1024.0 / 1024.0 / 1024.0;
        let disk_total_gb = disk_total_bytes as f64 / 1024.0 / 1024.0 / 1024.0;
        let disk_percent = if disk_total_gb > 0.0 {
            (disk_used_gb / disk_total_gb) * 100.0
        } else {
            0.0
        };

        // 3. Sample Network Traffic
        let networks = Networks::new_with_refreshed_list();
        let mut rx_bytes = 0u64;
        let mut tx_bytes = 0u64;
        for (_interface, network) in &networks {
            rx_bytes += network.received();
            tx_bytes += network.transmitted();
        }
        let net_rx_kbps = rx_bytes as f64 / 1024.0;
        let net_tx_kbps = tx_bytes as f64 / 1024.0;

        // 4. Save to local SQLite Monitor DB
        sqlx::query(
            r#"
            INSERT INTO system_metrics (
                server_id, cpu_usage, memory_used_mb, memory_total_mb, memory_percent,
                disk_used_gb, disk_total_gb, disk_percent, net_rx_kbps, net_tx_kbps, timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(1i64)
        .bind(cpu_usage)
        .bind(memory_used_mb)
        .bind(memory_total_mb)
        .bind(memory_percent)
        .bind(disk_used_gb)
        .bind(disk_total_gb)
        .bind(disk_percent)
        .bind(net_rx_kbps)
        .bind(net_tx_kbps)
        .bind(now)
        .execute(&self.pool)
        .await?;

        info!(
            "Recorded telemetry to Monitor DB (CPU: {:.1}%, RAM: {:.1}%)",
            cpu_usage, memory_percent
        );

        // 5. Stream over gRPC channel
        let grpc_req = SystemMetricsRequest {
            server_id: 1,
            cpu_usage,
            memory_used_mb,
            memory_total_mb,
            memory_percent,
            disk_used_gb,
            disk_total_gb,
            disk_percent,
            net_rx_kbps,
            net_tx_kbps,
            timestamp: now,
        };

        let mut lock = self.grpc_pusher.lock().await;
        if let Some(ref mut pusher) = *lock {
            let _ = pusher.stream_system_metrics(grpc_req).await;
        }

        Ok(())
    }
}
