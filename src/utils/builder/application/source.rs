use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::{ApplicationSpec, SourceSpec},
    exec::ExecResult,
    git::GitCli,
};
use tokio_util::sync::CancellationToken;

impl ApplicationBuilder {
    pub(super) async fn prepare_source(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        match &spec.source {
            SourceSpec::Docker { image, registry } => {
                if let Some(auth) = registry {
                    self.docker
                        .login(Some(&auth.registry), &auth.username, &auth.password)
                        .await?;
                    let pull = self.docker.image_pull_cancelled(&[image], cancel).await;
                    let logout = self.docker.logout(Some(&auth.registry)).await;
                    match (pull, logout) {
                        (Err(error), _) => return Err(error),
                        (Ok(_), Err(error)) => return Err(error),
                        (Ok(_), Ok(_)) => {}
                    }
                } else {
                    self.docker.image_pull_cancelled(&[image], cancel).await?;
                }
            }
            SourceSpec::Git {
                url,
                branch,
                submodules,
            } => {
                let git = GitCli::from_executor(self.executor.clone())
                    .with_repository(spec.work_directory.clone());
                let git_dir = format!("{}/.git", spec.work_directory);
                if self
                    .executor
                    .run("test", ["-d", git_dir.as_str()])
                    .await
                    .is_ok()
                {
                    git.fetch_cancelled(&["--prune", "origin", branch], cancel)
                        .await?;
                    git.reset(&["--hard", "FETCH_HEAD"]).await?;
                } else {
                    if let Some(parent) = std::path::Path::new(&spec.work_directory).parent() {
                        self.executor
                            .run("mkdir", ["-p", parent.to_string_lossy().as_ref()])
                            .await?;
                    }
                    GitCli::from_executor(self.executor.clone())
                        .clone_repository_cancelled(
                            url,
                            Some(&spec.work_directory),
                            &["--branch", branch, "--single-branch"],
                            cancel,
                        )
                        .await?;
                }
                if *submodules {
                    git.submodule(&["update", "--init", "--recursive"]).await?;
                }
            }
        }
        Ok(())
    }
}
