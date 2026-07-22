use prost::Message;
use tonic::transport::Channel;
use tracing::{error, info};

#[derive(Clone, PartialEq, Message)]
pub struct SystemMetricsRequest {
    #[prost(int64, tag = "1")]
    pub server_id: i64,
    #[prost(double, tag = "2")]
    pub cpu_usage: f64,
    #[prost(double, tag = "3")]
    pub memory_used_mb: f64,
    #[prost(double, tag = "4")]
    pub memory_total_mb: f64,
    #[prost(double, tag = "5")]
    pub memory_percent: f64,
    #[prost(double, tag = "6")]
    pub disk_used_gb: f64,
    #[prost(double, tag = "7")]
    pub disk_total_gb: f64,
    #[prost(double, tag = "8")]
    pub disk_percent: f64,
    #[prost(double, tag = "9")]
    pub net_rx_kbps: f64,
    #[prost(double, tag = "10")]
    pub net_tx_kbps: f64,
    #[prost(int64, tag = "11")]
    pub timestamp: i64,
}

#[derive(Clone, PartialEq, Message)]
pub struct ContainerMetricsRequest {
    #[prost(int64, tag = "1")]
    pub server_id: i64,
    #[prost(int64, tag = "2")]
    pub application_id: i64,
    #[prost(int64, tag = "3")]
    pub compose_id: i64,
    #[prost(string, tag = "4")]
    pub container_id: String,
    #[prost(string, tag = "5")]
    pub container_name: String,
    #[prost(double, tag = "6")]
    pub cpu_percent: f64,
    #[prost(double, tag = "7")]
    pub memory_used_mb: f64,
    #[prost(double, tag = "8")]
    pub memory_limit_mb: f64,
    #[prost(double, tag = "9")]
    pub net_rx_kbps: f64,
    #[prost(double, tag = "10")]
    pub net_tx_kbps: f64,
    #[prost(int64, tag = "11")]
    pub timestamp: i64,
}

#[derive(Clone, PartialEq, Message)]
pub struct MetricsAck {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(int64, tag = "2")]
    pub received_timestamp: i64,
    #[prost(string, tag = "3")]
    pub message: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct LogStreamRequest {
    #[prost(int64, tag = "1")]
    pub application_id: i64,
    #[prost(int64, tag = "2")]
    pub compose_id: i64,
    #[prost(string, tag = "3")]
    pub container_id: String,
    #[prost(int32, tag = "4")]
    pub tail_lines: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct LogChunk {
    #[prost(string, tag = "1")]
    pub container_id: String,
    #[prost(string, tag = "2")]
    pub log_line: String,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(bool, tag = "4")]
    pub is_stderr: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct HealthReportRequest {
    #[prost(int64, tag = "1")]
    pub target_id: i64,
    #[prost(string, tag = "2")]
    pub target_type: String,
    #[prost(string, tag = "3")]
    pub status: String,
    #[prost(int32, tag = "4")]
    pub response_time_ms: i32,
    #[prost(string, tag = "5")]
    pub error_message: String,
    #[prost(int64, tag = "6")]
    pub timestamp: i64,
}

#[derive(Clone, PartialEq, Message)]
pub struct HealthAck {
    #[prost(bool, tag = "1")]
    pub acknowledged: bool,
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
}

pub struct GrpcTelemetryPusher {
    channel: Option<Channel>,
    target_url: String,
}

impl GrpcTelemetryPusher {
    pub async fn connect(grpc_url: &str) -> Self {
        match Channel::from_shared(grpc_url.to_string()) {
            Ok(endpoint) => match endpoint.connect().await {
                Ok(channel) => {
                    info!("Connected real-time gRPC stream channel to {}", grpc_url);
                    Self {
                        channel: Some(channel),
                        target_url: grpc_url.to_string(),
                    }
                }
                Err(err) => {
                    error!("gRPC stream channel connection error to {}: {:?}", grpc_url, err);
                    Self {
                        channel: None,
                        target_url: grpc_url.to_string(),
                    }
                }
            },
            Err(err) => {
                error!("Invalid gRPC stream URL {}: {:?}", grpc_url, err);
                Self {
                    channel: None,
                    target_url: grpc_url.to_string(),
                }
            }
        }
    }

    pub async fn stream_system_metrics(&mut self, req: SystemMetricsRequest) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref channel) = self.channel {
            let mut client = tonic::client::Grpc::new(channel.clone());
            let path = http::uri::PathAndQuery::from_static("/monitoring.MonitoringService/StreamSystemMetrics");
            client.ready().await?;
            let _res: tonic::Response<MetricsAck> = client.unary(tonic::Request::new(req), path, tonic::codec::ProstCodec::default()).await?;
            info!("pushed system metrics to real-time gRPC stream channel");
        }
        Ok(())
    }

    pub async fn stream_container_metrics(&mut self, req: ContainerMetricsRequest) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref channel) = self.channel {
            let mut client = tonic::client::Grpc::new(channel.clone());
            let path = http::uri::PathAndQuery::from_static("/monitoring.MonitoringService/StreamContainerMetrics");
            client.ready().await?;
            let _res: tonic::Response<MetricsAck> = client.unary(tonic::Request::new(req), path, tonic::codec::ProstCodec::default()).await?;
            info!("pushed container metrics to real-time gRPC stream channel");
        }
        Ok(())
    }

    pub async fn stream_logs(&mut self, req: LogStreamRequest) -> Result<tonic::Streaming<LogChunk>, Box<dyn std::error::Error + Send + Sync>> {
        let channel = self.channel.as_ref().ok_or("gRPC channel not connected")?;
        let mut client = tonic::client::Grpc::new(channel.clone());
        let path = http::uri::PathAndQuery::from_static("/monitoring.MonitoringService/StreamLogs");
        client.ready().await?;
        let res = client.server_streaming(tonic::Request::new(req), path, tonic::codec::ProstCodec::default()).await?;
        Ok(res.into_inner())
    }

    pub async fn stream_health(&mut self, req: HealthReportRequest) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref channel) = self.channel {
            let mut client = tonic::client::Grpc::new(channel.clone());
            let path = http::uri::PathAndQuery::from_static("/monitoring.MonitoringService/StreamHealth");
            client.ready().await?;
            let _res: tonic::Response<HealthAck> = client.unary(tonic::Request::new(req), path, tonic::codec::ProstCodec::default()).await?;
            info!("pushed health report to real-time gRPC stream channel");
        }
        Ok(())
    }
}
