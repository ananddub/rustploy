use std::error::Error;
use tokio::sync::broadcast;
use tokio::sync::watch::{Receiver, Sender};
use tower::util::BoxCloneService;

pub type DynError = Box<dyn Error + Send + Sync + 'static>;
pub type DynService<Req, Res> = BoxCloneService<Req, Res, DynError>;

#[derive(Debug, Clone)]
pub struct AppDeploy {
    pub app_id: IdType,
    pub project_id: i64,
    pub env_id: i64,
    pub state: Sender<DeployState>,
    pub broadcast: broadcast::Sender<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdType {
    AppId(i64),
    ComposeId(i64),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeployState {
    Queue,
    GitClone,
    GitRetry,
    DockerImagePull,
    DockerImageBuild,
    DockerComposeUp,
    DockerRetry,
    Building,
    BuildingRetry,
    Deploying,
    Deployed,
    StoppedByUser,
    Failed(String),
}
