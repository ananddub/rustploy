use crate::utils::{
    docker::{
        core::{types::{NodeAvailability, NodeRole}, ArgBuilder},
        client::DockerCli,
        DockerOutput, DockerResult,
    },
};

// ── NodesHandle ─────────────────────────────────────────────────────────────

pub struct NodesHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> NodesHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn update(&self, node_id: impl Into<String>) -> NodeUpdateBuilder<'a> {
        NodeUpdateBuilder::new(self.cli, node_id)
    }

    pub fn promote(&self, node_id: impl Into<String>) -> NodePromoteBuilder<'a> {
        NodePromoteBuilder::new(self.cli, node_id)
    }

    pub fn demote(&self, node_id: impl Into<String>) -> NodeDemoteBuilder<'a> {
        NodeDemoteBuilder::new(self.cli, node_id)
    }

    pub fn remove(&self, node_id: impl Into<String>) -> NodeRemoveBuilder<'a> {
        NodeRemoveBuilder::new(self.cli, node_id)
    }

    pub fn list(&self) -> NodeListBuilder<'a> {
        NodeListBuilder::new(self.cli)
    }

    pub fn ps(&self, node_id: impl Into<String>) -> NodePsBuilder<'a> {
        NodePsBuilder::new(self.cli, node_id)
    }

    pub async fn inspect(&self, node_id: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.cli.run(["node", "inspect", node_id.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

// ── NodeUpdateBuilder ───────────────────────────────────────────────────────

pub struct NodeUpdateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodeUpdateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["node", "update"]), node_id: node_id.into() }
    }

    pub fn availability(mut self, avail: NodeAvailability) -> Self { self.args.pair("--availability", avail.to_string()); self }
    pub fn role(mut self, role: NodeRole) -> Self { self.args.pair("--role", role.to_string()); self }
    pub fn label(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self { self.args.pair("--label-add", format!("{}={}", key.as_ref(), value.as_ref())); self }
    pub fn remove_label(mut self, key: impl AsRef<str>) -> Self { self.args.pair("--label-rm", key); self }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NodeUpdateBuilder);

// ── NodePromoteBuilder ──────────────────────────────────────────────────────

pub struct NodePromoteBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodePromoteBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["node", "promote"]), node_id: node_id.into() }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NodePromoteBuilder);

// ── NodeDemoteBuilder ───────────────────────────────────────────────────────

pub struct NodeDemoteBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodeDemoteBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["node", "demote"]), node_id: node_id.into() }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NodeDemoteBuilder);

// ── NodeRemoveBuilder ───────────────────────────────────────────────────────

pub struct NodeRemoveBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodeRemoveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["node", "rm"]), node_id: node_id.into() }
    }

    pub fn force(mut self) -> Self { self.args.flag("--force"); self }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NodeRemoveBuilder);

// ── NodeListBuilder ─────────────────────────────────────────────────────────

pub struct NodeListBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> NodeListBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["node", "ls"]) }
    }

    pub fn filter(mut self, f: crate::utils::docker::query::filter::NodeFilter) -> Self { self.args.filter(f); self }
    pub fn filters(mut self, fs: impl IntoIterator<Item = crate::utils::docker::query::filter::NodeFilter>) -> Self {
        for f in fs { self.args.filter(f); }
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::NodeSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(NodeListBuilder);

// ── NodePsBuilder ───────────────────────────────────────────────────────────

pub struct NodePsBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodePsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["node", "ps"]), node_id: node_id.into() }
    }

    pub fn filter(mut self, f: crate::utils::docker::query::filter::TaskFilter) -> Self { self.args.filter(f); self }
    pub fn filters(mut self, fs: impl IntoIterator<Item = crate::utils::docker::query::filter::TaskFilter>) -> Self {
        for f in fs { self.args.filter(f); }
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::TaskSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.args.push(&self.node_id);
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(NodePsBuilder);
