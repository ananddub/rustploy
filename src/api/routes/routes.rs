use std::sync::Arc;

use auto_di::singleton;
use axum::Router;

use crate::api::routes::socket::Socket;

#[singleton]
pub async fn router_init(sock: Arc<Socket>) -> Router<()> {
    auto_route::routes()
        .await
        .expect("failed to build auto-registered controller routes")
        .merge(auto_route::openapi_routes("/openapi.json", "/swagger-ui"))
        .merge(auto_route::scalar_routes("/scalar", "/openapi.json"))
        .layer(sock.layer.clone())
}
