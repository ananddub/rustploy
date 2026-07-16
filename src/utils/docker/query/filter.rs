use std::fmt;
use crate::utils::docker::{NetworkDriver, VolumeDriver, NetworkScope, NetworkType};

// ── Container ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerStatus {
    Created,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead,
}

impl fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Created => "created",
            Self::Restarting => "restarting",
            Self::Running => "running",
            Self::Removing => "removing",
            Self::Paused => "paused",
            Self::Exited => "exited",
            Self::Dead => "dead",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Starting,
    Healthy,
    Unhealthy,
    None,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Starting => "starting",
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
            Self::None => "none",
        })
    }
}

/// Typesafe filter for `docker container ls --filter`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerFilter {
    Id(String),
    Name(String),
    Status(ContainerStatus),
    Health(HealthStatus),
    /// `label=KEY=VALUE`
    Label(String, String),
    /// `label=KEY` (key-only label presence check)
    LabelKey(String),
    Network(String),
    /// Filter by ancestor image name/id.
    Ancestor(String),
    Volume(String),
    /// Containers created after this container id/name.
    Since(String),
    /// Containers created before this container id/name.
    Before(String),
    /// Containers that exited with the given exit code.
    Exited(i32),
    /// Filter by published ports.
    Publish(String),
    /// Filter by exposed ports.
    Expose(String),
    /// Filter by isolation technology.
    Isolation(String),
    /// Filter containers that are swarm tasks.
    IsTask(bool),
}

impl ContainerFilter {
    pub fn id(v: impl Into<String>) -> Self { Self::Id(v.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn network(v: impl Into<String>) -> Self { Self::Network(v.into()) }
    pub fn ancestor(v: impl Into<String>) -> Self { Self::Ancestor(v.into()) }
    pub fn volume(v: impl Into<String>) -> Self { Self::Volume(v.into()) }
    pub fn since(v: impl Into<String>) -> Self { Self::Since(v.into()) }
    pub fn before(v: impl Into<String>) -> Self { Self::Before(v.into()) }
    pub fn publish(v: impl Into<String>) -> Self { Self::Publish(v.into()) }
    pub fn expose(v: impl Into<String>) -> Self { Self::Expose(v.into()) }
    pub fn isolation(v: impl Into<String>) -> Self { Self::Isolation(v.into()) }
    pub fn is_task(v: bool) -> Self { Self::IsTask(v) }
}

impl fmt::Display for ContainerFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Status(s) => write!(f, "status={s}"),
            Self::Health(h) => write!(f, "health={h}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Network(v) => write!(f, "network={v}"),
            Self::Ancestor(v) => write!(f, "ancestor={v}"),
            Self::Volume(v) => write!(f, "volume={v}"),
            Self::Since(v) => write!(f, "since={v}"),
            Self::Before(v) => write!(f, "before={v}"),
            Self::Exited(c) => write!(f, "exited={c}"),
            Self::Publish(v) => write!(f, "publish={v}"),
            Self::Expose(v) => write!(f, "expose={v}"),
            Self::Isolation(v) => write!(f, "isolation={v}"),
            Self::IsTask(b) => write!(f, "is-task={b}"),
        }
    }
}

// ── Image ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageFilter {
    /// Match against image name or `name:tag`.
    Reference(String),
    Label(String, String),
    LabelKey(String),
    Before(String),
    Since(String),
    Until(String),
    Dangling(bool),
}

impl ImageFilter {
    pub fn reference(v: impl Into<String>) -> Self { Self::Reference(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn before(v: impl Into<String>) -> Self { Self::Before(v.into()) }
    pub fn since(v: impl Into<String>) -> Self { Self::Since(v.into()) }
    pub fn until(v: impl Into<String>) -> Self { Self::Until(v.into()) }
}

impl fmt::Display for ImageFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reference(v) => write!(f, "reference={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Before(v) => write!(f, "before={v}"),
            Self::Since(v) => write!(f, "since={v}"),
            Self::Until(v) => write!(f, "until={v}"),
            Self::Dangling(b) => write!(f, "dangling={b}"),
        }
    }
}

// ── Service ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceMode {
    Replicated,
    Global,
}

impl fmt::Display for ServiceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Replicated => "replicated",
            Self::Global => "global",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceFilter {
    Id(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
    Mode(ServiceMode),
}

impl ServiceFilter {
    pub fn id(v: impl Into<String>) -> Self { Self::Id(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
}

impl fmt::Display for ServiceFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Mode(m) => write!(f, "mode={m}"),
        }
    }
}

// ── Network ──────────────────────────────────────────────────────────────────



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkFilter {
    Driver(NetworkDriver),
    Id(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
    Scope(NetworkScope),
    Type(NetworkType),
    Dangling(bool),
}

impl NetworkFilter {
    pub fn driver(v: impl Into<NetworkDriver>) -> Self { Self::Driver(v.into()) }
    pub fn id(v: impl Into<String>) -> Self { Self::Id(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
    pub fn dangling(v: bool) -> Self { Self::Dangling(v) }
}

impl fmt::Display for NetworkFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Driver(v) => write!(f, "driver={}", v.as_str()),
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Scope(s) => write!(f, "scope={s}"),
            Self::Type(t) => write!(f, "type={t}"),
            Self::Dangling(b) => write!(f, "dangling={b}"),
        }
    }
}

// ── Volume ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VolumeFilter {
    Driver(VolumeDriver),
    Label(String, String),
    LabelKey(String),
    Name(String),
    Dangling(bool),
}

impl VolumeFilter {
    pub fn driver(v: impl Into<VolumeDriver>) -> Self { Self::Driver(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
}

impl fmt::Display for VolumeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Driver(v) => write!(f, "driver={}", v.as_str()),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Dangling(b) => write!(f, "dangling={b}"),
        }
    }
}

// ── Node ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeMembership {
    Accepted,
    Pending,
}

impl fmt::Display for NodeMembership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeFilter {
    Id(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
    Role(crate::utils::docker::core::types::NodeRole),
    Membership(NodeMembership),
    NodeLabel(String, String),
    NodeLabelKey(String),
}

impl NodeFilter {
    pub fn id(v: impl Into<String>) -> Self { Self::Id(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
    pub fn membership(v: NodeMembership) -> Self { Self::Membership(v) }
    pub fn node_label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::NodeLabel(k.into(), v.into()) }
    pub fn node_label_key(k: impl Into<String>) -> Self { Self::NodeLabelKey(k.into()) }
}

impl fmt::Display for NodeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Role(r) => write!(f, "role={r}"),
            Self::Membership(m) => write!(f, "membership={m}"),
            Self::NodeLabel(k, v) => write!(f, "node.label={k}={v}"),
            Self::NodeLabelKey(k) => write!(f, "node.label={k}"),
        }
    }
}

// ── Secret ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecretFilter {
    Id(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
}

impl SecretFilter {
    pub fn id(v: impl Into<String>) -> Self { Self::Id(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
}

impl fmt::Display for SecretFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
        }
    }
}

// ── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigFilter {
    Id(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
}

impl ConfigFilter {
    pub fn id(v: impl Into<String>) -> Self { Self::Id(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
}

impl fmt::Display for ConfigFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
        }
    }
}

// ── Task ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskDesiredState {
    Running,
    Shutdown,
    Accepted,
}

impl fmt::Display for TaskDesiredState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Running => "running",
            Self::Shutdown => "shutdown",
            Self::Accepted => "accepted",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskFilter {
    Id(String),
    Label(String, String),
    Name(String),
    Node(String),
    DesiredState(TaskDesiredState),
    UpToDate(bool),
    IsTask(bool),
}

impl TaskFilter {
    pub fn id(v: impl Into<String>) -> Self { Self::Id(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
    pub fn node(v: impl Into<String>) -> Self { Self::Node(v.into()) }
    pub fn desired_state(v: TaskDesiredState) -> Self { Self::DesiredState(v) }
    pub fn up_to_date(v: bool) -> Self { Self::UpToDate(v) }
    pub fn is_task(v: bool) -> Self { Self::IsTask(v) }
}

impl fmt::Display for TaskFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Node(v) => write!(f, "node={v}"),
            Self::DesiredState(v) => write!(f, "desired-state={v}"),
            Self::UpToDate(b) => write!(f, "up-to-date={b}"),
            Self::IsTask(b) => write!(f, "is-task={b}"),
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use crate::utils::docker::DockerCli;
    use super::*;

    #[test]
    fn container_filter_display() {
        assert_eq!(ContainerFilter::Status(ContainerStatus::Running).to_string(), "status=running");
        assert_eq!(ContainerFilter::Health(HealthStatus::Healthy).to_string(), "health=healthy");
        assert_eq!(ContainerFilter::label("app", "api").to_string(), "label=app=api");
        assert_eq!(ContainerFilter::label_key("traefik.enable").to_string(), "label=traefik.enable");
        assert_eq!(ContainerFilter::Exited(0).to_string(), "exited=0");
        assert_eq!(ContainerFilter::ancestor("nginx:latest").to_string(), "ancestor=nginx:latest");
    }

    #[test]
    fn image_filter_display() {
        assert_eq!(ImageFilter::reference("api:*").to_string(), "reference=api:*");
        assert_eq!(ImageFilter::Dangling(true).to_string(), "dangling=true");
    }

    #[test]
    fn service_filter_display() {
        assert_eq!(ServiceFilter::Mode(ServiceMode::Replicated).to_string(), "mode=replicated");
        assert_eq!(ServiceFilter::name("myservice").to_string(), "name=myservice");
    }

    #[test]
    fn network_filter_display() {
        assert_eq!(NetworkFilter::Scope(NetworkScope::Swarm).to_string(), "scope=swarm");
        assert_eq!(NetworkFilter::Type(NetworkType::Custom).to_string(), "type=custom");
    }

    #[test]
    fn volume_filter_display() {
        assert_eq!(VolumeFilter::Dangling(false).to_string(), "dangling=false");
        assert_eq!(VolumeFilter::driver("local").to_string(), "driver=local");
    }

}
