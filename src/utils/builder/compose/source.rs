use super::{compose::ComposeBuilder, spec::ComposeSource};
use crate::utils::{builder::compose::spec::ComposeSpec, exec::ExecResult, git::GitCli};

impl ComposeBuilder {
    pub(super) async fn prepare_source(&self, spec: &ComposeSpec) -> ExecResult<()> {
        match &spec.source {
            ComposeSource::Raw { content } => {
                if let Some(parent) = std::path::Path::new(&spec.compose_path).parent() {
                    self.executor
                        .run("mkdir", ["-p", parent.to_string_lossy().as_ref()])
                        .await?;
                }
                self.write_file(&spec.compose_path, content.as_bytes())
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
                    git.fetch(&["--prune", "origin", branch]).await?;
                    git.reset(&["--hard", "FETCH_HEAD"]).await?;
                } else {
                    if let Some(parent) = std::path::Path::new(&spec.work_directory).parent() {
                        self.executor
                            .run("mkdir", ["-p", parent.to_string_lossy().as_ref()])
                            .await?;
                    }
                    GitCli::from_executor(self.executor.clone())
                        .clone_repository(
                            url,
                            Some(&spec.work_directory),
                            &["--branch", branch, "--single-branch"],
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
