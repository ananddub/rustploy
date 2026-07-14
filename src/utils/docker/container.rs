use super::{
    ContainerSummary, DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
};
use serde::de::DeserializeOwned;
use tokio::sync::mpsc;

impl DockerCli {
    /// Raw container list — prefer [`DockerCli::containers()`] handle instead.
    pub async fn containers_raw(
        &self,
        all: bool,
        filters: &[&str],
    ) -> DockerResult<Vec<ContainerSummary>> {
        let mut args = vec!["container", "ls", "--format", "{{json .}}"];
        if all {
            args.push("--all");
        }
        for filter in filters {
            args.extend(["--filter", filter]);
        }
        self.json_lines(&args).await
    }
    pub async fn container_create(&self, args: &[&str]) -> DockerResult<String> {
        Ok(self
            .prefixed(&["container", "create"], args)
            .await?
            .stdout
            .trim()
            .into())
    }
    pub async fn container_run(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["container", "run"], args).await
    }
    pub async fn container_start(&self, targets: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["container", "start"], targets).await
    }
    pub async fn container_stop(&self, targets: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["container", "stop"], targets).await
    }
    pub async fn container_restart(&self, targets: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["container", "restart"], targets).await
    }
    pub async fn container_kill(&self, targets: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["container", "kill"], targets).await
    }
    pub async fn container_pause(&self, targets: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["container", "pause"], targets).await
    }
    pub async fn container_unpause(&self, targets: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["container", "unpause"], targets).await
    }
    pub async fn container_remove(
        &self,
        targets: &[&str],
        force: bool,
        volumes: bool,
    ) -> DockerResult<DockerOutput> {
        let mut args = vec!["container", "rm"];
        if force {
            args.push("--force");
        }
        if volumes {
            args.push("--volumes");
        }
        args.extend_from_slice(targets);
        self.run(args).await
    }
    pub async fn container_inspect<T: DeserializeOwned>(
        &self,
        targets: &[&str],
    ) -> DockerResult<Vec<T>> {
        let mut args = vec!["container", "inspect"];
        args.extend_from_slice(targets);
        self.json(&args).await
    }
    pub async fn container_logs(
        &self,
        target: &str,
        tail: Option<usize>,
        timestamps: bool,
    ) -> DockerResult<String> {
        let tail = tail.map(|value| value.to_string());
        let mut args = vec!["container", "logs"];
        if timestamps {
            args.push("--timestamps");
        }
        if let Some(value) = tail.as_deref() {
            args.extend(["--tail", value]);
        }
        args.push(target);
        let output = self.run(args).await?;
        Ok(format!("{}{}", output.stdout, output.stderr))
    }
    pub async fn container_logs_stream(
        &self,
        target: &str,
        options: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut args = vec!["container", "logs"];
        args.extend_from_slice(options);
        args.push(target);
        self.run_stream(args, sender).await
    }
    pub async fn container_stats_stream(
        &self,
        target: &str,
        options: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut args = vec!["container", "stats"];
        args.extend_from_slice(options);
        args.push(target);
        self.run_stream(args, sender).await
    }
    pub async fn container_exec(&self, target: &str, args: &[&str]) -> DockerResult<DockerOutput> {
        let mut command = vec!["container", "exec", target];
        command.extend_from_slice(args);
        self.run(command).await
    }
    pub async fn container_exec_stream(
        &self,
        target: &str,
        args: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut command = vec!["container", "exec", target];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn container_prune(&self, filters: &[&str]) -> DockerResult<DockerOutput> {
        self.prune("container", filters).await
    }
}
