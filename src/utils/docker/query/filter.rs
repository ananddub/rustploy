use std::fmt;

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
    Dangling(bool),
}

impl ImageFilter {
    pub fn reference(v: impl Into<String>) -> Self { Self::Reference(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn before(v: impl Into<String>) -> Self { Self::Before(v.into()) }
    pub fn since(v: impl Into<String>) -> Self { Self::Since(v.into()) }
}

impl fmt::Display for ImageFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reference(v) => write!(f, "reference={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Before(v) => write!(f, "before={v}"),
            Self::Since(v) => write!(f, "since={v}"),
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
pub enum NetworkScope { Local, Swarm, Global }

impl fmt::Display for NetworkScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self { Self::Local => "local", Self::Swarm => "swarm", Self::Global => "global" })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkType { Custom, Builtin }

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self { Self::Custom => "custom", Self::Builtin => "builtin" })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkFilter {
    Driver(String),
    Id(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
    Scope(NetworkScope),
    Type(NetworkType),
}

impl NetworkFilter {
    pub fn driver(v: impl Into<String>) -> Self { Self::Driver(v.into()) }
    pub fn id(v: impl Into<String>) -> Self { Self::Id(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
}

impl fmt::Display for NetworkFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Driver(v) => write!(f, "driver={v}"),
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Scope(s) => write!(f, "scope={s}"),
            Self::Type(t) => write!(f, "type={t}"),
        }
    }
}

// ── Volume ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VolumeFilter {
    Driver(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
    Dangling(bool),
}

impl VolumeFilter {
    pub fn driver(v: impl Into<String>) -> Self { Self::Driver(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
}

impl fmt::Display for VolumeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Driver(v) => write!(f, "driver={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Dangling(b) => write!(f, "dangling={b}"),
        }
    }
}

// ── Node ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeFilter {
    Id(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
    Role(crate::utils::docker::core::types::NodeRole),
}

impl NodeFilter {
    pub fn id(v: impl Into<String>) -> Self { Self::Id(v.into()) }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self { Self::Label(k.into(), v.into()) }
    pub fn label_key(k: impl Into<String>) -> Self { Self::LabelKey(k.into()) }
    pub fn name(v: impl Into<String>) -> Self { Self::Name(v.into()) }
}

impl fmt::Display for NodeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Role(r) => write!(f, "role={r}"),
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

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
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
