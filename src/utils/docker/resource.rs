use super::{DockerCli, DockerOutput, DockerResult, NetworkSummary, VolumeSummary};
use serde::de::DeserializeOwned;

impl DockerCli {
    /// Raw network list — prefer [`DockerCli::networks()`] handle instead.
    pub async fn networks_raw(&self, filters: &[&str]) -> DockerResult<Vec<NetworkSummary>> {
        self.list("network", filters).await
    }
    /// Raw volume list — prefer [`DockerCli::volumes()`] handle instead.
    pub async fn volumes_raw(&self, filters: &[&str]) -> DockerResult<Vec<VolumeSummary>> {
        self.list("volume", filters).await
    }
    pub async fn network_create(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["network", "create"], args).await
    }
    pub async fn network_remove(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["network", "rm"], args).await
    }
    pub async fn network_connect(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["network", "connect"], args).await
    }
    pub async fn network_disconnect(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["network", "disconnect"], args).await
    }
    pub async fn volume_create(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["volume", "create"], args).await
    }
    pub async fn volume_remove(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["volume", "rm"], args).await
    }
    pub async fn network_inspect<T: DeserializeOwned>(
        &self,
        args: &[&str],
    ) -> DockerResult<Vec<T>> {
        let mut cmd = vec!["network", "inspect"];
        cmd.extend_from_slice(args);
        self.json(&cmd).await
    }
    pub async fn volume_inspect<T: DeserializeOwned>(&self, args: &[&str]) -> DockerResult<Vec<T>> {
        let mut cmd = vec!["volume", "inspect"];
        cmd.extend_from_slice(args);
        self.json(&cmd).await
    }
    pub async fn network_prune(&self, filters: &[&str]) -> DockerResult<DockerOutput> {
        self.prune("network", filters).await
    }
    pub async fn volume_prune(&self, filters: &[&str]) -> DockerResult<DockerOutput> {
        self.prune("volume", filters).await
    }
}
