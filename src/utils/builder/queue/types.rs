use crate::utils::builder::spec::BuilderEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Deploy,
    Redeploy,
    Reload,
    Start,
    Stop,
    Cancel,
}
