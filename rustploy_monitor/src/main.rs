mod containers;
mod db;
mod grpc_client;
mod grpc_server;
mod monitoring;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use db::Db;
use monitoring::ServerMetricsMonitor;
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    db: Arc<Db>,
    server_token: String,
}

#[derive(Deserialize)]
struct MetricsQuery {
    limit: Option<String>,
}

#[derive(Deserialize)]
struct ContainerMetricsQuery {
    #[serde(rename = "appName")]
    app_name: Option<String>,
    limit: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("rustploy_monitor=info".parse()?))
        .init();

    info!("Starting Dokploy-compatible Rustploy Dedicated Monitoring Service...");

    let db_url = std::env::var("MONITOR_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://monitor.db".to_string());

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3001);

    let server_token = std::env::var("METRICS_TOKEN").unwrap_or_default();
    let callback_url = std::env::var("METRICS_URL_CALLBACK").unwrap_or_default();
    let cpu_threshold = std::env::var("CPU_THRESHOLD")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0);
    let mem_threshold = std::env::var("MEMORY_THRESHOLD")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0);
    let refresh_rate = std::env::var("REFRESH_RATE")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(60);

    let db = Arc::new(Db::init(&db_url).await?);
    let state = AppState {
        db: db.clone(),
        server_token: server_token.clone(),
    };

    let server_monitor = Arc::new(ServerMetricsMonitor::new());

    // 1. Background loop for Server Metrics collection & Alert threshold checks
    let db_clone = db.clone();
    let monitor_clone = server_monitor.clone();
    let cb_url = callback_url.clone();
    let token = server_token.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(refresh_rate));
        loop {
            interval.tick().await;
            let metric = monitor_clone.get_server_metrics();
            if let Err(err) = db_clone.save_server_metric(&metric).await {
                error!("Error saving server metrics to SQLite: {:?}", err);
            }
            monitor_clone.check_thresholds(&metric, cpu_threshold, mem_threshold, &cb_url, &token, "DOKPLOY").await;
        }
    });

    // 2. Background loop for Docker Container Metrics collection
    let db_clone2 = db.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(refresh_rate));
        loop {
            interval.tick().await;
            let container_metrics = containers::collect_docker_container_metrics();
            for c_metric in container_metrics {
                if let Err(err) = db_clone2.save_container_metric(&c_metric).await {
                    error!("Error saving container metric: {:?}", err);
                }
            }
        }
    });

    // 3. Background metrics cleanup task (Runs every 24h)
    let db_clone3 = db.clone();
    let retention_days = std::env::var("RETENTION_DAYS")
        .ok()
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(7);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400));
        loop {
            interval.tick().await;
            if let Ok(affected) = db_clone3.cleanup_old_metrics(retention_days).await {
                info!("Cleaned up {} old metrics records older than {} days", affected, retention_days);
            }
        }
    });

    // 4. Router & REST API Handlers matching Dokploy Specification
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .route("/metrics/containers", get(container_metrics_handler))
        .with_state(state);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Dedicated Monitoring Service listening on http://0.0.0.0:{}", port);

    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

async fn metrics_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<MetricsQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    verify_auth(&headers, &state.server_token)?;

    let limit_num = match query.limit.as_deref() {
        Some("all") => 10000,
        Some(val) => val.parse::<i64>().unwrap_or(50),
        None => 50,
    };

    let metrics = state
        .db
        .get_last_n_server_metrics(limit_num)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(metrics))
}

async fn container_metrics_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ContainerMetricsQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    verify_auth(&headers, &state.server_token)?;

    let app_name = match query.app_name {
        Some(ref name) if !name.is_empty() => name,
        _ => return Ok(Json(serde_json::json!([]))),
    };

    let limit_num = match query.limit.as_deref() {
        Some("all") => 10000,
        Some(val) => val.parse::<i64>().unwrap_or(50),
        None => 50,
    };

    let metrics = state
        .db
        .get_last_n_container_metrics(app_name, limit_num)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!(metrics)))
}

fn verify_auth(headers: &HeaderMap, token: &str) -> Result<(), (StatusCode, String)> {
    if token.is_empty() {
        return Ok(());
    }

    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            let token_val = auth_str.strip_prefix("Bearer ").unwrap_or(auth_str);
            if token_val == token {
                return Ok(());
            }
        }
    }

    Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))
}
