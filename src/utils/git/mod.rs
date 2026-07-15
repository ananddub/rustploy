pub use client::GitCli;
pub use types::{GitBranch, GitStatusEntry};

pub mod client;
pub mod handles;
pub mod provider;
pub mod types;

pub use provider::{GitProvider, GitProviderBuilder};
