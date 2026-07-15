use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::{ApplicationSpec, BuilderEvent, SourceSpec},
    exec::{ExecError, ExecResult},
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
                    self.ctx.docker
                        .login(Some(&auth.registry), &auth.username, &auth.password)
                        .await?;
                    let pull = self.ctx.docker.images().pull(image.as_str()).cancel_with(cancel.clone()).pull().await;
                    let logout = self.ctx.docker.logout(Some(&auth.registry)).await;
                    match (pull, logout) {
                        (Err(error), _) => return Err(error),
                        (Ok(_), Err(error)) => return Err(error),
                        (Ok(_), Ok(_)) => {}
                    }
                } else {
                    self.ctx.docker.images().pull(image.as_str()).cancel_with(cancel.clone()).pull().await?;
                }
            }
            SourceSpec::Git {
                url,
                branch,
                submodules,
            } => {
                let git = GitCli::from_executor(self.ctx.executor.clone())
                    .with_repository(spec.work_directory.clone());
                let branch = resolve_branch(&GitCli::from_executor(self.ctx.executor.clone()), url, branch, cancel)
                    .await?;
                let git_dir = format!("{}/.git", spec.work_directory);
                if self
                    .ctx.executor
                    .run("test", ["-d", git_dir.as_str()])
                    .await
                    .is_ok()
                {
                    self.ctx.emit(BuilderEvent::Message(format!(
                        "fetching {url} branch {branch} into {}",
                        spec.work_directory
                    )))
                    .await;
                    git.remote(&["set-url", "origin", url]).await?;
                    git.fetch_raw_cancelled(&["--prune", "origin", branch.as_str()], cancel)
                        .await?;
                    git.reset(&["--hard", "FETCH_HEAD"]).await?;
                } else {
                    if let Some(parent) = std::path::Path::new(&spec.work_directory).parent() {
                        self.ctx.executor
                            .run("mkdir", ["-p", parent.to_string_lossy().as_ref()])
                            .await?;
                    }
                    self.ctx.emit(BuilderEvent::Message(format!(
                        "cloning {url} branch {branch} into {}",
                        spec.work_directory
                    )))
                    .await;
                    GitCli::from_executor(self.ctx.executor.clone())
                        .clone_repository_raw_cancelled(
                            url,
                            Some(&spec.work_directory),
                            &["--branch", branch.as_str(), "--single-branch"],
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

async fn resolve_branch(
    git: &GitCli,
    url: &str,
    branch: &Option<String>,
    cancel: &CancellationToken,
) -> ExecResult<String> {
    if let Some(branch) = branch.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
        return Ok(branch.to_owned());
    }
    git.remote_default_branch_cancelled(url, cancel)
        .await?
        .ok_or_else(|| ExecError::CommandFailed {
            code: None,
            stderr: format!("could not detect default branch for repository {url}"),
        })
}
