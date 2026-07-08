use std::sync::Arc;

use auto_di::singleton;
use axum::Router;

use crate::api::routes::socket::Socket;

#[singleton]
pub async fn router_init(sock: Arc<Socket>) -> Router<()> {
    auto_route::routes()
        .await
        .expect("failed to build auto-registered controller routes")
        .layer(sock.layer.clone())
}

pub mod socket;
