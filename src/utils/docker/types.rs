use serde::Deserialize;

macro_rules! docker_row {
    ($name:ident { $($field:ident : $rename:literal),* $(,)? }) => {
        #[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
        pub struct $name { $(#[serde(rename = $rename, default)] pub $field: String,)* }
    };
}

docker_row!(ContainerSummary {
    id: "ID",
    image: "Image",
    command: "Command",
    created_at: "CreatedAt",
    running_for: "RunningFor",
    ports: "Ports",
    status: "Status",
    size: "Size",
    names: "Names",
    labels: "Labels",
    mounts: "Mounts",
    networks: "Networks",
    state: "State"
});
docker_row!(ImageSummary {
    id: "ID",
    repository: "Repository",
    tag: "Tag",
    digest: "Digest",
    created_at: "CreatedAt",
    created_since: "CreatedSince",
    size: "Size",
    shared_size: "SharedSize",
    unique_size: "UniqueSize",
    containers: "Containers"
});
docker_row!(NetworkSummary {
    id: "ID",
    name: "Name",
    driver: "Driver",
    scope: "Scope",
    ipv6: "IPv6",
    internal: "Internal",
    labels: "Labels",
    created_at: "CreatedAt"
});
docker_row!(VolumeSummary {
    name: "Name",
    driver: "Driver",
    scope: "Scope",
    mountpoint: "Mountpoint",
    labels: "Labels",
    availability: "Availability",
    group: "Group",
    links: "Links",
    size: "Size"
});
docker_row!(ServiceSummary {
    id: "ID",
    name: "Name",
    mode: "Mode",
    replicas: "Replicas",
    image: "Image",
    ports: "Ports"
});
docker_row!(NodeSummary {
    id: "ID",
    hostname: "Hostname",
    status: "Status",
    availability: "Availability",
    manager_status: "ManagerStatus",
    engine_version: "EngineVersion"
});
docker_row!(StackSummary {
    name: "Name",
    services: "Services",
    orchestrator: "Orchestrator",
    namespace: "Namespace"
});
docker_row!(TaskSummary {
    id: "ID",
    name: "Name",
    image: "Image",
    node: "Node",
    desired_state: "DesiredState",
    current_state: "CurrentState",
    error: "Error",
    ports: "Ports"
});
docker_row!(SecretSummary {
    id: "ID",
    name: "Name",
    driver: "Driver",
    created_at: "CreatedAt",
    updated_at: "UpdatedAt",
    labels: "Labels"
});
docker_row!(ConfigSummary {
    id: "ID",
    name: "Name",
    created_at: "CreatedAt",
    updated_at: "UpdatedAt",
    labels: "Labels"
});

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComposeContainer {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub project: String,
    #[serde(default)]
    pub service: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub health: String,
    #[serde(default)]
    pub exit_code: i64,
    #[serde(default)]
    pub publishers: Vec<ComposePublisher>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComposePublisher {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub target_port: i64,
    #[serde(default)]
    pub published_port: i64,
    #[serde(default)]
    pub protocol: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerVersion {
    #[serde(default)]
    pub client: DockerVersionComponent,
    #[serde(default)]
    pub server: DockerVersionComponent,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerVersionComponent {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub api_version: String,
    #[serde(default)]
    pub git_commit: String,
    #[serde(default)]
    pub go_version: String,
    #[serde(default)]
    pub os: String,
    #[serde(default)]
    pub arch: String,
}

pub type DockerInfo = serde_json::Map<String, serde_json::Value>;
pub type DockerDiskUsage = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmInfo {
    #[serde(rename = "NodeID", default)]
    pub node_id: String,
    #[serde(default)]
    pub node_addr: String,
    #[serde(default)]
    pub local_node_state: String,
    #[serde(default)]
    pub control_available: bool,
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub remote_managers: Option<Vec<SwarmRemoteManager>>,
    #[serde(default)]
    pub nodes: u32,
    #[serde(default)]
    pub managers: u32,
    pub cluster: Option<SwarmClusterInfo>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmRemoteManager {
    #[serde(rename = "NodeID", default)]
    pub node_id: String,
    #[serde(default)]
    pub addr: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmClusterInfo {
    #[serde(rename = "ID", default)]
    pub id: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub data_path_port: u16,
    #[serde(default)]
    pub subnet_size: u8,
}
