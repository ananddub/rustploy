use std::sync::Arc;

use auto_di::singleton;
use bollard::Docker;

use crate::config::init::Config;

#[singleton]
pub async fn init_docker(config: Arc<Config>) -> Docker {
    Docker::connect_with_host(&config.clone().socket_path).unwrap()
}
