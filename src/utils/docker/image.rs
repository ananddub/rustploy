use super::{
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent, ImageSummary,
};
use serde::de::DeserializeOwned;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

impl DockerCli {
    pub async fn images(&self, all: bool, filters: &[&str]) -> DockerResult<Vec<ImageSummary>> {
        let mut args = vec!["image", "ls", "--format", "{{json .}}"];
        if all {
            args.push("--all");
        }
        for filter in filters {
            args.extend(["--filter", filter]);
        }
        self.json_lines(&args).await
    }
    pub async fn image_pull(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "pull"], args).await
    }
    pub async fn image_pull_cancelled(
        &self,
        args: &[&str],
        cancel: &CancellationToken,
    ) -> DockerResult<DockerOutput> {
        self.prefixed_cancelled(&["image", "pull"], args, cancel)
            .await
    }
    /// Logs in with stdin, pulls the image, then removes the temporary registry session.
    pub async fn image_pull_authenticated(
        &self,
        registry: &str,
        username: &str,
        password: &str,
        image: &str,
        options: &[&str],
    ) -> DockerResult<DockerOutput> {
        self.login(Some(registry), username, password).await?;
        let mut args = options.to_vec();
        args.push(image);
        let pull = self.image_pull(&args).await;
        let logout = self.logout(Some(registry)).await;
        match (pull, logout) {
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
            (Ok(output), Ok(_)) => Ok(output),
        }
    }
    pub async fn image_pull_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut command = vec!["image", "pull"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn image_pull_authenticated_stream(
        &self,
        registry: &str,
        username: &str,
        password: &str,
        image: &str,
        options: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        self.login(Some(registry), username, password).await?;
        let mut command = vec!["image", "pull"];
        command.extend_from_slice(options);
        command.push(image);
        let pull = self.run_stream(command, sender).await;
        let logout = self.logout(Some(registry)).await;
        match (pull, logout) {
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
            (Ok(status), Ok(_)) => Ok(status),
        }
    }
    pub async fn image_push(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "push"], args).await
    }
    pub async fn image_push_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut command = vec!["image", "push"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn image_build(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "build"], args).await
    }
    pub async fn image_build_cancelled(
        &self,
        args: &[&str],
        cancel: &CancellationToken,
    ) -> DockerResult<DockerOutput> {
        self.prefixed_cancelled(&["image", "build"], args, cancel)
            .await
    }
    pub async fn image_build_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut command = vec!["image", "build"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn image_tag(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "tag"], args).await
    }
    pub async fn image_load(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "load"], args).await
    }
    pub async fn image_save(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "save"], args).await
    }
    pub async fn image_history(&self, args: &[&str]) -> DockerResult<DockerOutput> {
        self.prefixed(&["image", "history"], args).await
    }
    pub async fn image_remove(&self, images: &[&str], force: bool) -> DockerResult<DockerOutput> {
        let mut args = vec!["image", "rm"];
        if force {
            args.push("--force");
        }
        args.extend_from_slice(images);
        self.run(args).await
    }
    pub async fn image_inspect<T: DeserializeOwned>(
        &self,
        targets: &[&str],
    ) -> DockerResult<Vec<T>> {
        let mut args = vec!["image", "inspect"];
        args.extend_from_slice(targets);
        self.json(&args).await
    }
    pub async fn image_prune(&self, filters: &[&str]) -> DockerResult<DockerOutput> {
        self.prune("image", filters).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::docker::DockerError;
    use std::os::unix::fs::PermissionsExt;

    #[tokio::test]
    async fn authenticated_pull_uses_stdin_and_logs_out_after_pull_failure() {
        let dir = tempfile::tempdir().unwrap();
        let executable = dir.path().join("docker");
        let calls = dir.path().join("calls");
        let script = format!(
            "#!/bin/sh\necho \"$*\" >> '{}'\nif [ \"$1\" = login ]; then read password; [ \"$password\" = secret ] || exit 9; fi\nif [ \"$1 $2\" = 'image pull' ]; then echo pull-failed >&2; exit 7; fi\n",
            calls.display()
        );
        std::fs::write(&executable, script).unwrap();
        let mut permissions = std::fs::metadata(&executable).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&executable, permissions).unwrap();
        let docker = DockerCli::with_executable(executable);
        let error = docker
            .image_pull_authenticated(
                "registry.example",
                "alice",
                "secret",
                "registry.example/app:latest",
                &[],
            )
            .await
            .unwrap_err();
        assert!(matches!(
            error,
            DockerError::CommandFailed { code: Some(7), .. }
        ));
        let calls = std::fs::read_to_string(calls).unwrap();
        assert!(calls.contains("login --username alice --password-stdin registry.example"));
        assert!(calls.contains("image pull registry.example/app:latest"));
        assert!(calls.contains("logout registry.example"));
        assert!(!calls.contains("secret"));
    }
}
