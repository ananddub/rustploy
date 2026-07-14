use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResolveImage {
    Always,
    Changed,
    Never,
}

impl fmt::Display for ResolveImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolveImage::Always => write!(f, "always"),
            ResolveImage::Changed => write!(f, "changed"),
            ResolveImage::Never => write!(f, "never"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeAvailability {
    Active,
    Pause,
    Drain,
}

impl fmt::Display for NodeAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeAvailability::Active => write!(f, "active"),
            NodeAvailability::Pause => write!(f, "pause"),
            NodeAvailability::Drain => write!(f, "drain"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeRole {
    Worker,
    Manager,
}

impl fmt::Display for NodeRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeRole::Worker => write!(f, "worker"),
            NodeRole::Manager => write!(f, "manager"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwarmRole {
    Worker,
    Manager,
}
