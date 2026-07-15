use reqwest::{Client, RequestBuilder};

#[derive(Clone)]
pub struct ProviderClient {
    pub(crate) client: Client,
    pub(crate) token: Option<String>,
}

impl ProviderClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: Client::builder()
                .user_agent("rustploy-builder")
                .build()
                .unwrap_or_default(),
            token,
        }
    }

    /// Helper to inject bearer token if present
    pub fn authenticate(&self, mut req: RequestBuilder) -> RequestBuilder {
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }
        req
    }
}
