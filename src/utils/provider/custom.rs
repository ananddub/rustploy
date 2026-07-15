use super::{sync::ProviderSyncBuilder, types::CloneProtocol};

/// The main entry point for a Custom Git provider client (raw URL).
pub struct CustomClient {
    url: String,
}

impl CustomClient {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Access repository-specific endpoints.
    pub fn repository(&self) -> CustomRepoBuilder<'_> {
        CustomRepoBuilder { url: &self.url }
    }
}

pub struct CustomRepoBuilder<'a> {
    url: &'a str,
}

impl<'a> CustomRepoBuilder<'a> {
    /// Get the clone URL for this repository
    /// Note: A custom URL implies the user provided the exact URL, so protocol may be ignored.
    pub fn clone_url(&self, _protocol: CloneProtocol) -> String {
        self.url.to_string()
    }

    /// Creates a sync builder to fetch or clone this repository into a local path.
    pub fn sync_into(&self, destination: &'a str, protocol: CloneProtocol) -> ProviderSyncBuilder<'a> {
        ProviderSyncBuilder::new(self.clone_url(protocol), destination)
    }
}
