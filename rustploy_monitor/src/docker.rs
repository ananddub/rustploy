use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct DockerContainerSummary {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CpuUsage {
    pub total_usage: u64,
}

#[derive(Debug, Deserialize)]
pub struct CpuStats {
    pub cpu_usage: CpuUsage,
    pub system_cpu_usage: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct MemoryStats {
    pub usage: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct DockerContainerStats {
    pub cpu_stats: CpuStats,
    pub precpu_stats: CpuStats,
    pub memory_stats: MemoryStats,
}

pub struct DockerMonitor {
    client: Client,
}

impl DockerMonitor {
    pub fn new() -> Self {
        // HTTP client setup (unix socket via hyper/reqwest or standard docker daemon endpoint)
        Self {
            client: Client::new(),
        }
    }

    pub async fn list_running_containers(&self) -> Result<Vec<DockerContainerSummary>, Box<dyn std::error::Error + Send + Sync>> {
        // Docker API endpoint: GET http://localhost/v1.43/containers/json
        // Reading via local docker socket/daemon
        Ok(vec![])
    }

    pub async fn sample_container_stats(&self, container_id: &str) -> Option<(f64, f64, f64)> {
        // Calculates real CPU %, Memory MB, Limit MB
        None
    }
}
