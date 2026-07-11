use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::{ApplicationSpec, MountKind},
    exec::{ExecError, ExecResult},
};

impl ApplicationBuilder {
    pub(super) async fn prepare_file_mounts(&self, spec: &ApplicationSpec) -> ExecResult<()> {
        for mount in &spec.mounts {
            if matches!(mount.kind, MountKind::File) {
                let parent = std::path::Path::new(&mount.source)
                    .parent()
                    .ok_or_else(|| ExecError::CommandFailed {
                        code: None,
                        stderr: "invalid file mount source".into(),
                    })?;
                self.executor
                    .run("mkdir", ["-p", parent.to_string_lossy().as_ref()])
                    .await?;
                let content = mount
                    .content
                    .as_deref()
                    .ok_or_else(|| ExecError::CommandFailed {
                        code: None,
                        stderr: format!("file mount {} has no content", mount.target),
                    })?;
                self.write_file(&mount.source, content.as_bytes()).await?;
            }
        }
        Ok(())
    }
}
