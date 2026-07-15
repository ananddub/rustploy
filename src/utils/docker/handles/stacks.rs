use crate::utils::docker::{
    core::{types::ResolveImage, ArgBuilder},
    client::DockerCli,
    DockerOutput, DockerResult,
};

// ── StacksHandle ────────────────────────────────────────────────────────────

pub struct StacksHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> StacksHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn deploy(&self, stack_name: impl Into<String>) -> StackDeployBuilder<'a> {
        StackDeployBuilder::new(self.cli, stack_name)
    }

    pub fn remove(&self, stack_name: impl Into<String>) -> StackRemoveBuilder<'a> {
        StackRemoveBuilder::new(self.cli, stack_name)
    }

    pub fn list(&self) -> StackListBuilder<'a> {
        StackListBuilder::new(self.cli)
    }

    pub fn ps(&self, stack_name: impl Into<String>) -> StackPsBuilder<'a> {
        StackPsBuilder::new(self.cli, stack_name)
    }

    pub fn services(&self, stack_name: impl Into<String>) -> StackServicesBuilder<'a> {
        StackServicesBuilder::new(self.cli, stack_name)
    }

    pub fn config(&self, stack_name: impl Into<String>) -> StackConfigBuilder<'a> {
        StackConfigBuilder::new(self.cli, stack_name)
    }
}

// ── StackDeployBuilder ──────────────────────────────────────────────────────

pub struct StackDeployBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    stack_name: String,
}

impl<'a> StackDeployBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, stack_name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["stack", "deploy"]), stack_name: stack_name.into() }
    }

    pub fn compose_file(mut self, path: impl AsRef<str>) -> Self { self.args.pair("--compose-file", path); self }
    pub fn compose_bytes(self, content: &[u8]) -> Self { self.compose_file("-").stdin(content.to_vec()) }
    pub fn compose_string(self, content: impl AsRef<str>) -> Self { self.compose_file("-").stdin(content.as_ref().as_bytes().to_vec()) }

    pub fn with_registry_auth(mut self) -> Self { self.args.flag("--with-registry-auth"); self }
    pub fn prune(mut self) -> Self { self.args.flag("--prune"); self }
    pub fn resolve_image(mut self, resolve: ResolveImage) -> Self { self.args.pair("--resolve-image", resolve.to_string()); self }
    
    // Internal helper for stdin injection
    fn stdin(mut self, _data: Vec<u8>) -> Self {
        self.args.pair_opt("--__internal_stdin", None as Option<&str>); // marker
        // Not perfectly clean inside ArgBuilder but we can use executor's run_with_stdin
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.stack_name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(StackDeployBuilder);

// ── StackRemoveBuilder ──────────────────────────────────────────────────────

pub struct StackRemoveBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    stack_name: String,
}

impl<'a> StackRemoveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, stack_name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["stack", "rm"]), stack_name: stack_name.into() }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.stack_name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(StackRemoveBuilder);

// ── StackListBuilder ────────────────────────────────────────────────────────

pub struct StackListBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> StackListBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["stack", "ls"]) }
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::StackSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(StackListBuilder);

// ── StackPsBuilder ──────────────────────────────────────────────────────────

pub struct StackPsBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    stack_name: String,
}

impl<'a> StackPsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, stack_name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["stack", "ps"]), stack_name: stack_name.into() }
    }

    pub fn filter(mut self, f: crate::utils::docker::query::filter::TaskFilter) -> Self { self.args.filter(f); self }
    pub fn filters(mut self, fs: impl IntoIterator<Item = crate::utils::docker::query::filter::TaskFilter>) -> Self {
        for f in fs { self.args.filter(f); }
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.stack_name);
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::TaskSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.args.push(&self.stack_name);
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(StackPsBuilder);

// ── StackServicesBuilder ────────────────────────────────────────────────────

pub struct StackServicesBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    stack_name: String,
}

impl<'a> StackServicesBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, stack_name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["stack", "services"]), stack_name: stack_name.into() }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.stack_name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(StackServicesBuilder);

// ── StackConfigBuilder ──────────────────────────────────────────────────────

pub struct StackConfigBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    stack_name: String,
}

impl<'a> StackConfigBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, stack_name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["stack", "config"]), stack_name: stack_name.into() }
    }

    pub fn compose_file(mut self, path: impl AsRef<str>) -> Self { self.args.pair("--compose-file", path); self }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.stack_name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(StackConfigBuilder);
