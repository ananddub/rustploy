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
        self.cli.run(&self.args.build()).await
    }
}

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
        self.cli.run(&self.args.build()).await
    }
}

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
        self.cli.run(&self.args.build()).await
    }
}

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
        self.cli.run(&self.args.build()).await
    }
}

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
        self.cli.run(&self.args.build()).await
    }
}
