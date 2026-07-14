use std::error::Error;
use tokio::sync::broadcast;
use tokio::sync::watch::Sender;
use tokio_util::sync::CancellationToken;
use tower::util::BoxCloneService;

pub type DynError = Box<dyn Error + Send + Sync + 'static>;
pub type DynService<Req, Res> = BoxCloneService<Req, Res, DynError>;

#[derive(Debug, Clone)]
pub struct AppDeploy {
    pub app_id: IdType,
    pub project_id: i64,
    pub env_id: i64,
    pub state: Sender<DeployState>,
    pub broadcast: broadcast::Sender<DeployEvent>,
    pub cancellation_token: CancellationToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveDeploySnapshot {
    pub id: IdType,
    pub project_id: i64,
    pub env_id: i64,
    pub state: DeployState,
}

#[derive(Debug)]
pub struct DeploySubscription {
    pub state: tokio::sync::watch::Receiver<DeployState>,
    pub events: broadcast::Receiver<DeployEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdType {
    AppId(i64),
    ComposeId(i64),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeployState {
    // Waiting
    Queue,
    Preparing,

    // Git
    GitClone,
    GitRetry,
    GitSuccess,

    // Docker Image
    DockerImagePull,
    DockerImageBuild,
    DockerBuildRetry,
    DockerBuildSuccess,

    // Docker Compose
    DockerComposePull,
    DockerComposeDown,
    DockerComposeUp,
    DockerComposeRestart,
    DockerComposeRetry,

    // Container
    ContainerCreating,
    ContainerStarting,
    ContainerRunning,
    ContainerStopping,
    ContainerStopped,
    ContainerRestarting,

    // Deployment
    Building,
    BuildingRetry,
    BuildSuccess,

    Deploying,
    HealthCheck,
    WaitingForHealthy,

    // Final States
    Deployed,
    Cancelled,
    StoppedByUser,

    // Rollback
    RollingBack,
    RollbackSuccess,
    RollbackFailed,

    // Cleanup
    CleaningUp,
    CleanupComplete,
    Failed(String),

    RecoverAfterRestart,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeployEvent {
    StateChanged(DeployState),
    Log(String),
    Message(String),
}
