use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::{ApplicationSpec, MountKind},
    exec::{ExecError, ExecResult},
};
use tokio_util::sync::CancellationToken;

impl ApplicationBuilder {
    pub(super) async fn prepare_file_mounts(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        for mount in &spec.mounts {
            if matches!(mount.kind, MountKind::File) {
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
                    .await?;
            }
        }
        Ok(())
    }
}
