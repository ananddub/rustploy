use crate::utils::exec::{ArgBuilder, CommandExecutor, ExecOutput, ExecResult};
use tokio_util::sync::CancellationToken;
use crate::utils::docker::DockerCli;
use crate::utils::docker::handles::RestartPolicy;
use crate::utils::docker::query::{ContainerFilter, ContainerStatus};

#[derive(Clone, Debug)]
pub struct RailpackCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> RailpackCli<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    pub fn prepare(&self, path: impl Into<String>) -> RailpackPrepareBuilder<'_> {
        let mut args = ArgBuilder::cmd(&["prepare"]);
        args.push(path.into());
        RailpackPrepareBuilder {
            executor: self.executor,
            args,
        }
    }
}

pub struct RailpackPrepareBuilder<'a> {
    executor: &'a CommandExecutor,
    args: ArgBuilder,
}

impl<'a> RailpackPrepareBuilder<'a> {
    pub fn plan_out(mut self, path: impl Into<String>) -> Self {
        self.args.pair("--plan-out", path.into());
        self
    }

    pub fn info_out(mut self, path: impl Into<String>) -> Self {
        self.args.pair("--info-out", path.into());
        self
    }

    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.pair("--env", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }

    pub fn previous(mut self, prev: impl Into<String>) -> Self {
        self.args.pair("--previous", prev.into());
        self
    }

    pub fn build_cmd(mut self, cmd: impl Into<String>) -> Self {
        self.args.pair("--build-cmd", cmd.into());
        self
    }

    pub fn start_cmd(mut self, cmd: impl Into<String>) -> Self {
        self.args.pair("--start-cmd", cmd.into());
        self
    }

    pub fn config_file(mut self, file: impl Into<String>) -> Self {
        self.args.pair("--config-file", file.into());
        self
    }

    pub fn error_missing_start(mut self) -> Self {
        self.args.flag("--error-missing-start");
        self
    }

    pub async fn run(self, cancel: &CancellationToken) -> ExecResult<ExecOutput> {
        let docker = DockerCli::from_executor(self.executor.clone());
        let name = "rustploy_buildkit".to_string();
        let is_running = docker.containers()
            .ps()
            .filters([
                ContainerFilter::Name(name.clone()),
                ContainerFilter::Status(ContainerStatus::Running)
            ])
            .exists()
            .await
            .unwrap_or(false);

        if !is_running {
            let _ = docker.containers()
                .create("moby/buildkit")
                .name(name.clone())
                .detach()
                .restart(RestartPolicy::Always)
                .privileged()
                .run()
                .await;
        }else {
            let _ = docker.containers().start(name.clone()).run().await;
        }

        self.executor.run_cancelled("railpack", self.args.build(), cancel).await
    }
}
