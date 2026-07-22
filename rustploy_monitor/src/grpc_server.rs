use crate::db::Db;
use crate::grpc_client::{
    ContainerMetricsRequest, HealthAck, HealthReportRequest, LogChunk, LogStreamRequest,
    MetricsAck, SystemMetricsRequest,
};
use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use tracing::info;

pub struct MonitoringGrpcServer {
    db: Arc<Db>,
}

impl MonitoringGrpcServer {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    // 1. StreamSystemMetrics handler
    pub async fn handle_stream_system_metrics(
        &self,
        req: Request<SystemMetricsRequest>,
    ) -> Result<Response<MetricsAck>, Status> {
        let msg = req.into_inner();
        info!(
            "gRPC Stream Received - Server ID: {}, CPU: {:.2}%, RAM: {:.2}%",
            msg.server_id, msg.cpu_usage, msg.memory_percent
        );

        Ok(Response::new(MetricsAck {
            success: true,
            received_timestamp: msg.timestamp,
            message: "System metric real-time stream point acknowledged".to_string(),
        }))
    }

    // 2. StreamContainerMetrics handler
    pub async fn handle_stream_container_metrics(
        &self,
        req: Request<ContainerMetricsRequest>,
    ) -> Result<Response<MetricsAck>, Status> {
        let msg = req.into_inner();
        info!(
            "gRPC Stream Received - Container: {}, CPU: {:.2}%",
            msg.container_name, msg.cpu_percent
        );

        Ok(Response::new(MetricsAck {
            success: true,
            received_timestamp: msg.timestamp,
            message: "Container metric real-time stream point acknowledged".to_string(),
        }))
    }

    // 3. StreamLogs handler (Server-streaming real-time log chunks)
    pub async fn handle_stream_logs(
        &self,
        req: Request<LogStreamRequest>,
    ) -> Result<Response<Pin<Box<dyn Stream<Item = Result<LogChunk, Status>> + Send>>>, Status> {
        let msg = req.into_inner();
        info!(
            "gRPC Log Stream Requested for Container ID: {}, App ID: {}",
            msg.container_id, msg.application_id
        );

        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let container_id = msg.container_id.clone();

        tokio::spawn(async move {
            let chunk = LogChunk {
                container_id: container_id.clone(),
                log_line: format!("Streaming real-time logs for container {}", container_id),
                timestamp: chrono::Utc::now().timestamp(),
                is_stderr: false,
            };
            let _ = tx.send(Ok(chunk)).await;
        });

        let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream) as Pin<Box<dyn Stream<Item = Result<LogChunk, Status>> + Send>>))
    }

    // 4. StreamHealth handler
    pub async fn handle_stream_health(
        &self,
        req: Request<HealthReportRequest>,
    ) -> Result<Response<HealthAck>, Status> {
        let msg = req.into_inner();
        info!(
            "gRPC Health Stream Received - Target: {} ({}), Status: {}",
            msg.target_id, msg.target_type, msg.status
        );

        Ok(Response::new(HealthAck {
            acknowledged: true,
            timestamp: msg.timestamp,
        }))
    }
}
