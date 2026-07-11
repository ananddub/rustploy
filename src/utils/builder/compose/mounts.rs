use super::compose::ComposeBuilder;
use crate::utils::{
    builder::{
        compose::spec::ComposeSpec,
        spec::{MountKind, MountSpec},
    },
    exec::{ExecError, ExecResult},
};
use tokio_util::sync::CancellationToken;

impl ComposeBuilder {
    pub(super) async fn prepare_runtime_files(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if let Some(parent) = std::path::Path::new(&spec.env_file).parent() {
            self.executor
                .run_cancelled("mkdir", ["-p", parent.to_string_lossy().as_ref()], cancel)
                .await?;
        }
        self.write_file_cancelled(&spec.env_file, env_file(spec).as_bytes(), cancel)
            .await?;
        for mount in &spec.mounts {
            self.prepare_file_mount(mount, cancel).await?;
        }
        Ok(())
    }

    async fn prepare_file_mount(
        &self,
        mount: &MountSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if !matches!(mount.kind, MountKind::File) {
            return Ok(());
        }
        let parent = std::path::Path::new(&mount.source)
            .parent()
            .ok_or_else(|| ExecError::CommandFailed {
                code: None,
                stderr: "invalid file mount source".into(),
            })?;
        self.executor
            .run_cancelled("mkdir", ["-p", parent.to_string_lossy().as_ref()], cancel)
            .await?;
        let content = mount
            .content
            .as_deref()
            .ok_or_else(|| ExecError::CommandFailed {
                code: None,
                stderr: format!("file mount {} has no content", mount.target),
            })?;
        self.write_file_cancelled(&mount.source, content.as_bytes(), cancel)
            .await
    }
}

fn env_file(spec: &ComposeSpec) -> String {
    spec.environment
        .iter()
        .map(|(key, value)| format!("{key}={value}\n"))
        .collect()
}
