use crate::utils::docker::{DockerCli, DockerError};
use crate::utils::docker::query::filter::{ContainerFilter, ContainerStatus};
use crate::utils::exec::{CommandExecutor, ExecError, ExecResult, ExecOutput};
use super::{dumper::DatabaseDumper, destination::S3Destination};
use tokio_util::sync::CancellationToken;

pub struct BackupRunner<'a> {
    executor: &'a CommandExecutor,
    dumper: &'a DatabaseDumper,
    destination: &'a S3Destination,
}

impl<'a> BackupRunner<'a> {
    pub fn new(
        executor: &'a CommandExecutor,
        dumper: &'a DatabaseDumper,
        destination: &'a S3Destination,
    ) -> Self {
        Self { executor, dumper, destination }
    }

    fn docker(&self) -> DockerCli {
        DockerCli::from_executor(self.executor.clone())
    }

    async fn resolve_container_id(&self) -> ExecResult<String> {
        let service_name = self.dumper.service_name();

        let label_key = self.dumper.container_label();

        let containers = self.docker()
            .containers()
            .ps()
            .filter(ContainerFilter::Status(ContainerStatus::Running))
            .filter(ContainerFilter::Label(
                label_key.into(),
                service_name.into(),
            ))
            .list()
            .await
            .map_err(docker_to_exec_error)?;

        let container = containers.into_iter().next().ok_or_else(|| ExecError::CommandFailed {
            code: Some(1),
            stderr: format!("no running container found for service '{service_name}'"),
        })?;

        Ok(container.id)
    }

    async fn verify_connection(&self, container_id: &str) -> ExecResult<()> {
        let check_cmd = self.dumper.connection_check_command();

        let out = self.docker()
            .containers()
            .exec(container_id)
            .interactive()
            .run(["sh", "-c", &check_cmd])
            .await
            .map_err(docker_to_exec_error)?;

        if !out.success() {
            return Err(ExecError::CommandFailed {
                code: out.status.code(),
                stderr: format!("connection check failed: {}", out.stderr),
            });
        }
        Ok(())
    }

    pub async fn run(
        &self,
        object_key: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let container_id = self.resolve_container_id().await?;

        self.verify_connection(&container_id).await?;

        let inner_cmd   = self.dumper.inner_dump_command();
        let rclone_args = self.destination.rclone_upload_args(object_key);
        let rclone_cmd  = format!("rclone {}", rclone_args.join(" "));

        let pipeline = format!(
            "set -eo pipefail; docker exec -i {id} sh -c {inner} | {rclone}",
            id    = container_id,
            inner = shell_single_quote(&inner_cmd),
            rclone = rclone_cmd,
        );

        self.executor
            .run_cancelled("sh", ["-c", &pipeline], cancel)
            .await
    }

    pub async fn run_restore(
        &self,
        object_key: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let container_id = self.resolve_container_id().await?;

        self.verify_connection(&container_id).await?;

        let inner_cmd   = self.dumper.inner_restore_command();
        let rclone_args = self.destination.rclone_cat_args(object_key);
        let rclone_cmd  = format!("rclone {}", rclone_args.join(" "));

        let pipeline = format!(
            "set -eo pipefail; {rclone} | docker exec -i {id} sh -c {inner}",
            rclone = rclone_cmd,
            id    = container_id,
            inner = shell_single_quote(&inner_cmd),
        );

        self.executor
            .run_cancelled("sh", ["-c", &pipeline], cancel)
            .await
    }
}

fn docker_to_exec_error(e: DockerError) -> ExecError {
    ExecError::CommandFailed { code: None, stderr: e.to_string() }
}

fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
