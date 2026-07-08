use std::sync::Arc;

use auto_di::singleton;
use bollard::Docker;

use crate::core::config::Config;

#[singleton]
pub async fn init_docker(config: Arc<Config>) -> Docker {
    Docker::connect_with_host(&config.clone().socket_path).unwrap()
}
