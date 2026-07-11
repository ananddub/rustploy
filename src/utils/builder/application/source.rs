use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::{ApplicationSpec, SourceSpec},
    exec::ExecResult,
    git::GitCli,
};

impl ApplicationBuilder {
    pub(super) async fn prepare_source(&self, spec: &ApplicationSpec) -> ExecResult<()> {
        match &spec.source {
            SourceSpec::Docker { image, registry } => {
                if let Some(auth) = registry {
                    self.docker
                        .image_pull_authenticated(
                            &auth.registry,
                            &auth.username,
                            &auth.password,
                            image,
                            &[],
                        )
                        .await?;
                } else {
                    self.docker.image_pull(&[image]).await?;
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
