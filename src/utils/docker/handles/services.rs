use crate::utils::{
    docker::{
        core::ArgBuilder,
        client::DockerCli,
        query::filter::ServiceFilter,
        DockerOutput, DockerResult, DockerExitStatus, DockerStreamEvent,
    },
};
use tokio::sync::mpsc;

// ── ServicesHandle ──────────────────────────────────────────────────────────

pub struct ServicesHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> ServicesHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn list(&self) -> ServiceListBuilder<'a> {
        ServiceListBuilder::new(self.cli)
    }

    pub fn update(&self, name: impl Into<String>) -> ServiceUpdateBuilder<'a> {
        ServiceUpdateBuilder::new(self.cli, name)
    }

    pub fn create(&self, image: impl Into<String>) -> ServiceCreateBuilder<'a> {
        ServiceCreateBuilder::new(self.cli, image)
    }

    pub fn remove(&self, name: impl Into<String>) -> ServiceRemoveBuilder<'a> {
        ServiceRemoveBuilder::new(self.cli, name)
    }

    pub fn ps(&self, name: impl Into<String>) -> ServicePsBuilder<'a> {
        ServicePsBuilder::new(self.cli, name)
    }

    pub fn logs(&self, name: impl Into<String>) -> ServiceLogsBuilder<'a> {
        ServiceLogsBuilder::new(self.cli, name)
    }

    pub fn scale(&self) -> ServiceScaleBuilder<'a> {
        ServiceScaleBuilder::new(self.cli)
    }

    pub fn rollback(&self, name: impl Into<String>) -> ServiceRollbackBuilder<'a> {
        ServiceRollbackBuilder::new(self.cli, name)
    }

    pub async fn inspect(&self, name: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.cli.run(["service", "inspect", name.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

// ── ServiceListBuilder ──────────────────────────────────────────────────────

pub struct ServiceListBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> ServiceListBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["service", "ls"]) }
    }

    pub fn filter(mut self, f: ServiceFilter) -> Self { self.args.filter(f); self }
    pub fn filters(mut self, fs: impl IntoIterator<Item = ServiceFilter>) -> Self {
        for f in fs { self.args.filter(f); }
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::ServiceSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceListBuilder);

// ── ServiceUpdateBuilder ────────────────────────────────────────────────────

pub struct ServiceUpdateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServiceUpdateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["service", "update"]), name: name.into() }
    }

    pub fn image(mut self, v: impl AsRef<str>) -> Self { self.args.pair("--image", v); self }
    pub fn replicas(mut self, n: u32) -> Self { self.args.pair("--replicas", n.to_string()); self }
    pub fn force(mut self) -> Self { self.args.flag("--force"); self }
    pub fn label_add(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.pair("--label-add", format!("{}={}", k.as_ref(), v.as_ref())); self }
    pub fn label_rm(mut self, k: impl AsRef<str>) -> Self { self.args.pair("--label-rm", k); self }
    pub fn env_add(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.pair("--env-add", format!("{}={}", k.as_ref(), v.as_ref())); self }
    pub fn env_rm(mut self, k: impl AsRef<str>) -> Self { self.args.pair("--env-rm", k); self }
    pub fn limit_memory(mut self, v: impl AsRef<str>) -> Self { self.args.pair("--limit-memory", v); self }
    pub fn update_parallelism(mut self, n: u32) -> Self { self.args.pair("--update-parallelism", n.to_string()); self }
    pub fn update_delay(mut self, v: impl AsRef<str>) -> Self { self.args.pair("--update-delay", v); self }
    
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.args.push(v);
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceUpdateBuilder);

// ── ServiceCreateBuilder ────────────────────────────────────────────────────

pub struct ServiceCreateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    image: String,
}

impl<'a> ServiceCreateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["service", "create"]), image: image.into() }
    }

    pub fn name(mut self, v: impl AsRef<str>) -> Self { self.args.pair("--name", v); self }
    pub fn replicas(mut self, n: u32) -> Self { self.args.pair("--replicas", n.to_string()); self }
    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.pair("--env", format!("{}={}", k.as_ref(), v.as_ref())); self }
    
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.args.push(v);
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.image);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceCreateBuilder);

// ── ServiceRemoveBuilder ────────────────────────────────────────────────────

pub struct ServiceRemoveBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServiceRemoveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["service", "rm"]), name: name.into() }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceRemoveBuilder);

// ── ServicePsBuilder ────────────────────────────────────────────────────────

pub struct ServicePsBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServicePsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["service", "ps"]), name: name.into() }
    }

    pub fn filter(mut self, f: crate::utils::docker::query::filter::TaskFilter) -> Self { self.args.filter(f); self }
    pub fn filters(mut self, fs: impl IntoIterator<Item = crate::utils::docker::query::filter::TaskFilter>) -> Self {
        for f in fs { self.args.filter(f); }
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::TaskSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.args.push(&self.name);
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(ServicePsBuilder);

// ── ServiceLogsBuilder ──────────────────────────────────────────────────────

pub struct ServiceLogsBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServiceLogsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["service", "logs"]), name: name.into() }
    }

    pub fn follow(mut self) -> Self { self.args.flag("--follow"); self }
    pub fn tail(mut self, n: impl AsRef<str>) -> Self { self.args.pair("--tail", n); self }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }
    
    pub async fn stream(mut self, sender: mpsc::Sender<DockerStreamEvent>) -> DockerResult<DockerExitStatus> {
        self.args.push(&self.name);
        self.cli.execute_stream(&self.args, sender).await
    }
}
crate::impl_builder_opts!(ServiceLogsBuilder);

// ── ServiceScaleBuilder ─────────────────────────────────────────────────────

pub struct ServiceScaleBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> ServiceScaleBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["service", "scale"]) }
    }

    pub fn service(mut self, name: impl AsRef<str>, replicas: u32) -> Self {
        self.args.push(format!("{}={}", name.as_ref(), replicas));
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceScaleBuilder);

// ── ServiceRollbackBuilder ──────────────────────────────────────────────────

pub struct ServiceRollbackBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServiceRollbackBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["service", "rollback"]), name: name.into() }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceRollbackBuilder);

impl crate::utils::exec::pipeline::IntoCommand for ServiceListBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}

impl crate::utils::exec::pipeline::IntoCommand for ServiceUpdateBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}

impl crate::utils::exec::pipeline::IntoCommand for ServiceCreateBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.image);
        a.preview()
    }
}

impl crate::utils::exec::pipeline::IntoCommand for ServiceRemoveBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}

impl crate::utils::exec::pipeline::IntoCommand for ServicePsBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}

impl crate::utils::exec::pipeline::IntoCommand for ServiceLogsBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}

impl crate::utils::exec::pipeline::IntoCommand for ServiceScaleBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}

impl crate::utils::exec::pipeline::IntoCommand for ServiceRollbackBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}
