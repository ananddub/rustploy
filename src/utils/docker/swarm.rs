use super::{
    ConfigSummary, DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
    NodeSummary, SecretSummary, ServiceSummary, StackSummary,
};
use serde::de::DeserializeOwned;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

impl DockerCli {
    pub async fn swarm_init_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["swarm", "init"], args).await
    }
    pub async fn swarm_join_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["swarm", "join"], args).await
    }
    pub async fn swarm_leave_raw(&self, force: bool) -> DockerResult<DockerOutput> {
        if force {
            self.run(["swarm", "leave", "--force"]).await
        } else {
            self.run(["swarm", "leave"]).await
        }
    }
    pub async fn swarm_update_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["swarm", "update"], args).await
    }
    pub async fn swarm_join_token_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["swarm", "join-token"], args).await
    }
    /// Raw service list — prefer [`DockerCli::services()`] handle instead.
    pub async fn services_raw(&self, filters: &[&str]) -> DockerResult<Vec<ServiceSummary>> {
        self.list("service", filters).await
    }




    pub async fn service_create(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["service", "create"], args).await
    }
    pub async fn service_update(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["service", "update"], args).await
    }
    pub async fn service_scale(&self, args: &[&str]) -> DockerResult<DockerOutput> {

        self.prefixed(&["service", "scale"], args).await
    }
    pub async fn service_remove(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["service", "rm"], args).await
    }
    pub async fn service_rollback(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["service", "rollback"], args).await
    }
    pub async fn service_logs(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["service", "logs"], args).await
    }
    pub async fn service_logs_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut command = vec!["service", "logs"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn service_ps(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["service", "ps"], args).await
    }
    pub async fn service_inspect<T: DeserializeOwned>(
        &self,
        args: &[&str],
    ) -> DockerResult<Vec<T>> {
        let mut cmd = vec!["service", "inspect"];
        cmd.extend_from_slice(args);
        self.json(&cmd).await
    }
    pub async fn nodes_raw(&self, filters: &[&str]) -> DockerResult<Vec<NodeSummary>> {
        self.list("node", filters).await
    }
    pub async fn node_update_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["node", "update"], args).await
    }
    pub async fn node_promote_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["node", "promote"], args).await
    }
    pub async fn node_demote_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["node", "demote"], args).await
    }
    pub async fn node_remove_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["node", "rm"], args).await
    }
    pub async fn stacks_raw(&self) -> DockerResult<Vec<StackSummary>> {
        self.list("stack", &[]).await
    }
    pub async fn stack_deploy_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["stack", "deploy"], args).await
    }
    pub async fn stack_deploy_raw_cancelled(
        &self,
        args: &[&str],
        cancel: &CancellationToken,
    ) -> DockerResult<DockerOutput> {
        self.prefixed_cancelled(&["stack", "deploy"], args, cancel)
            .await
    }
    pub async fn stack_deploy_raw_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut command = vec!["stack", "deploy"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn stack_remove_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["stack", "rm"], args).await
    }
    pub async fn stack_ps_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["stack", "ps"], args).await
    }
    pub async fn stack_services_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["stack", "services"], args).await
    }
    pub async fn secrets_raw(&self, filters: &[&str]) -> DockerResult<Vec<SecretSummary>> {
        self.list("secret", filters).await
    }
    pub async fn secret_create_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["secret", "create"], args).await
    }
    pub async fn secret_remove_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["secret", "rm"], args).await
    }
    pub async fn configs_raw(&self, filters: &[&str]) -> DockerResult<Vec<ConfigSummary>> {
        self.list("config", filters).await
    }
    pub async fn config_create_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["config", "create"], args).await
    }
    pub async fn config_remove_raw(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["config", "rm"], args).await
    }
}
