use super::{compose::ComposeBuilder, spec::ComposeSource};
use crate::utils::{
    builder::{compose::spec::ComposeSpec, spec::BuilderEvent},
    exec::ExecResult,
    git::GitCli,
};
use tokio_util::sync::CancellationToken;

impl ComposeBuilder {
    pub(super) async fn prepare_source(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        match &spec.source {
            ComposeSource::Raw { content } => {
                if let Some(parent) = std::path::Path::new(&spec.compose_path).parent() {
                    self.executor
                        .run_cancelled("mkdir", ["-p", parent.to_string_lossy().as_ref()], cancel)
                        .await?;
                }
                self.write_file_cancelled(&spec.compose_path, content.as_bytes(), cancel)
                    .await?;
            }
            ComposeSource::Git {
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
                    self.emit(BuilderEvent::Message(format!(
                        "fetching compose source {url} branch {branch} into {}",
                        spec.work_directory
                    )))
                    .await;
                    git.remote(&["set-url", "origin", url]).await?;
                    git.fetch_raw_cancelled(&["--prune", "origin", branch], cancel)
                        .await?;
                    git.reset(&["--hard", "FETCH_HEAD"]).await?;
                } else {
                    if let Some(parent) = std::path::Path::new(&spec.work_directory).parent() {
                        self.executor
                            .run_cancelled(
                                "mkdir",
                                ["-p", parent.to_string_lossy().as_ref()],
                                cancel,
                            )
                            .await?;
                    }
                    self.emit(BuilderEvent::Message(format!(
                        "cloning compose source {url} branch {branch} into {}",
                        spec.work_directory
                    )))
                    .await;
                    GitCli::from_executor(self.executor.clone())
                        .clone_repository_raw_cancelled(
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
