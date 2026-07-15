use super::{client::ProviderClient, sync::ProviderSyncBuilder, types::{CloneProtocol, WebhookEvent}};
use reqwest::Method;

/// The main entry point for the GitHub API client.
pub struct GithubClient {
    client: ProviderClient,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: ProviderClient::new(token),
        }
    }

    /// Access repository-specific endpoints.
    pub fn repository<'a>(&'a self, owner: &'a str, repo: &'a str) -> GithubRepoBuilder<'a> {
        GithubRepoBuilder {
            client: &self.client,
            owner,
            repo,
        }
    }
}

pub struct GithubRepoBuilder<'a> {
    client: &'a ProviderClient,
    owner: &'a str,
    repo: &'a str,
}

impl<'a> GithubRepoBuilder<'a> {
    /// Get the clone URL for this repository based on the protocol
    pub fn clone_url(&self, protocol: CloneProtocol) -> String {
        match protocol {
            CloneProtocol::Https => format!("https://github.com/{}/{}.git", self.owner, self.repo),
            CloneProtocol::Ssh => format!("git@github.com:{}/{}.git", self.owner, self.repo),
        }
    }

    /// Creates a sync builder to fetch or clone this repository into a local path.
    pub fn sync_into(&self, destination: &'a str, protocol: CloneProtocol) -> ProviderSyncBuilder<'a> {
        ProviderSyncBuilder::new(self.clone_url(protocol), destination)
    }

    /// Retrieve repository info.
    pub async fn get(&self) -> Result<String, String> {
        let url = format!("https://api.github.com/repos/{}/{}", self.owner, self.repo);
        let req = self.client.authenticate(self.client.client.request(Method::GET, url));
        
        req.send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())
    }

    /// Access webhook-specific endpoints for this repository.
    pub fn webhooks(&self) -> GithubWebhookBuilder<'a> {
        GithubWebhookBuilder {
            client: self.client,
            owner: self.owner,
            repo: self.repo,
            webhook_url: None,
            events: vec![],
            active: true,
        }
    }
}

pub struct GithubWebhookBuilder<'a> {
    client: &'a ProviderClient,
    owner: &'a str,
    repo: &'a str,
    webhook_url: Option<&'a str>,
    events: Vec<WebhookEvent>,
    active: bool,
}

impl<'a> GithubWebhookBuilder<'a> {
    /// Provide the URL where GitHub should send webhook payloads.
    pub fn create(mut self, url: &'a str) -> Self {
        self.webhook_url = Some(url);
        self
    }

    /// Specify the events the webhook should listen to.
    pub fn events(mut self, events: Vec<WebhookEvent>) -> Self {
        self.events = events;
        self
    }

    /// Activate or deactivate the webhook.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Execute the creation of the webhook on GitHub.
    pub async fn run(self) -> Result<String, String> {
        let target_url = self.webhook_url.ok_or_else(|| "Webhook URL is required to create a webhook".to_string())?;
        
        let url = format!("https://api.github.com/repos/{}/{}/hooks", self.owner, self.repo);
        
        let event_strings: Vec<&str> = self.events.iter().map(|e| e.as_github_event()).collect();
        
        let payload = serde_json::json!({
            "name": "web",
            "active": self.active,
            "events": if event_strings.is_empty() { vec!["push"] } else { event_strings },
            "config": {
                "url": target_url,
                "content_type": "json",
                "insecure_ssl": "0"
            }
        });

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
