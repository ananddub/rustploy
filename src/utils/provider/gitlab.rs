use super::{client::ProviderClient, sync::ProviderSyncBuilder, types::{CloneProtocol, WebhookEvent}};
use reqwest::Method;

/// The main entry point for the GitLab API client.
pub struct GitlabClient {
    client: ProviderClient,
}

impl GitlabClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: ProviderClient::new(token),
        }
    }

    /// Access repository-specific endpoints. 
    /// Note: GitLab APIs often require the URL-encoded project path instead of owner/repo.
    pub fn repository<'a>(&'a self, owner: &'a str, repo: &'a str) -> GitlabRepoBuilder<'a> {
        GitlabRepoBuilder {
            client: &self.client,
            owner,
            repo,
        }
    }
}

pub struct GitlabRepoBuilder<'a> {
    client: &'a ProviderClient,
    owner: &'a str,
    repo: &'a str,
}

impl<'a> GitlabRepoBuilder<'a> {
    fn project_id(&self) -> String {
        format!("{}%2F{}", self.owner, self.repo)
    }

    /// Get the clone URL for this repository based on the protocol
    pub fn clone_url(&self, protocol: CloneProtocol) -> String {
        match protocol {
            CloneProtocol::Https => format!("https://gitlab.com/{}/{}.git", self.owner, self.repo),
            CloneProtocol::Ssh => format!("git@gitlab.com:{}/{}.git", self.owner, self.repo),
        }
    }

    /// Creates a sync builder to fetch or clone this repository into a local path.
    pub fn sync_into(&self, destination: &'a str, protocol: CloneProtocol) -> ProviderSyncBuilder<'a> {
        ProviderSyncBuilder::new(self.clone_url(protocol), destination)
    }

    /// Retrieve repository info.
    pub async fn get(&self) -> Result<String, String> {
        let url = format!("https://gitlab.com/api/v4/projects/{}", self.project_id());
        let req = self.client.authenticate(self.client.client.request(Method::GET, url));
        
        req.send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())
    }

    /// Access webhook-specific endpoints for this repository.
    pub fn webhooks(&self) -> GitlabWebhookBuilder<'a> {
        GitlabWebhookBuilder {
            client: self.client,
            project_id: self.project_id(),
            webhook_url: None,
            events: vec![],
        }
    }
}

pub struct GitlabWebhookBuilder<'a> {
    client: &'a ProviderClient,
    project_id: String,
    webhook_url: Option<&'a str>,
    events: Vec<WebhookEvent>,
}

impl<'a> GitlabWebhookBuilder<'a> {
    /// Provide the URL where GitLab should send webhook payloads.
    pub fn create(mut self, url: &'a str) -> Self {
        self.webhook_url = Some(url);
        self
    }

    /// Specify the events the webhook should listen to.
    pub fn events(mut self, events: Vec<WebhookEvent>) -> Self {
        self.events = events;
        self
    }

    /// Execute the creation of the webhook on GitLab.
    pub async fn run(self) -> Result<String, String> {
        let target_url = self.webhook_url.ok_or_else(|| "Webhook URL is required to create a webhook".to_string())?;
        
        let url = format!("https://gitlab.com/api/v4/projects/{}/hooks", self.project_id);
        
        let mut payload = serde_json::Map::new();
        payload.insert("url".to_string(), serde_json::json!(target_url));
        payload.insert("enable_ssl_verification".to_string(), serde_json::json!(false));

        if self.events.is_empty() {
            payload.insert("push_events".to_string(), serde_json::json!(true));
        } else {
            for event in self.events {
                payload.insert(event.as_gitlab_event().to_string(), serde_json::json!(true));
            }
        }

        let req = self.client.authenticate(
            self.client.client.post(url).json(&payload)
        );

        req.send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())
    }
}
