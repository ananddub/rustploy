use auto_di::resolve;
use axum::Router;
use rustploy::{core::config::Config, core::logs::init_logs};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_logs();
    let service: Arc<Router> = resolve::<Router<()>>().await.unwrap();
    let port = resolve::<Config>().await.unwrap().port.clone();
    let host = resolve::<Config>().await.unwrap().host.clone();

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    let svc = service.as_ref().to_owned();

    tracing::info!("Listening on {}:{}", host, port);
    axum::serve(listener, svc).await.unwrap();
}
