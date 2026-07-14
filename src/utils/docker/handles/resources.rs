use crate::utils::docker::{
    core::ArgBuilder,
    query::filter::{NetworkFilter, VolumeFilter},
    DockerCli, DockerOutput, DockerResult, NetworkSummary, VolumeSummary,
};

// ═══════════════════════════ NETWORKS ═══════════════════════════════════════

pub struct NetworkHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> NetworkHandle<'a> {
    pub fn list(&self)                          -> NetworkQuery<'_>  { NetworkQuery::new(self.0) }
    pub fn create(&self, name: impl Into<String>) -> NetworkCreate<'_> { NetworkCreate::new(self.0, name) }
    pub fn prune(&self)                         -> NetworkPrune<'_>  { NetworkPrune::new(self.0) }
}

pub struct NetworkQuery<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> NetworkQuery<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["network", "ls", "--format", "{{json .}}"]) }
    }
    pub fn filter(mut self, f: NetworkFilter) -> Self { self.args.filter(f); self }
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn list(self) -> DockerResult<Vec<NetworkSummary>> {
        self.cli.execute_json_lines(&self.args).await
    }
    pub async fn exists(self) -> DockerResult<bool> { Ok(!self.list().await?.is_empty()) }
}
crate::impl_builder_opts!(NetworkQuery);

pub struct NetworkCreate<'a> { cli: &'a DockerCli, name: String, args: ArgBuilder }

impl<'a> NetworkCreate<'a> {
    fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self { cli, name: name.into(), args: ArgBuilder::default() }
    }
    pub fn driver(mut self, v: impl Into<String>)  -> Self { self.args.pair("--driver", v.into()); self }
    pub fn subnet(mut self, v: impl Into<String>)  -> Self { self.args.pair("--subnet", v.into()); self }
    pub fn gateway(mut self, v: impl Into<String>) -> Self { self.args.pair("--gateway", v.into()); self }
    pub fn label(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.label(k, v); self }
    pub fn opt(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.pair("--opt", format!("{}={}", k.as_ref(), v.as_ref())); self
    }
    pub fn attachable(mut self) -> Self { self.args.flag("--attachable"); self }
    pub fn internal(mut self)   -> Self { self.args.flag("--internal"); self }
    pub fn ipv6(mut self)       -> Self { self.args.flag("--ipv6"); self }
    pub fn scope(mut self, v: impl Into<String>) -> Self { self.args.pair("--scope", v.into()); self }
    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["network", "create"]);
        a.push_all(self.args.clone().build());
        a.push(&self.name);
        a.preview()
    }
    pub async fn run(self) -> DockerResult<String> {
        let mut a = ArgBuilder::cmd(&["network", "create"]);
        a.push_all(self.args.build());
        a.push(&self.name);
        Ok(self.cli.execute(&a).await?.stdout.trim().to_string())
    }
}
crate::impl_builder_opts!(NetworkCreate);

pub struct NetworkPrune<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> NetworkPrune<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["network", "prune", "--force"]) }
    }
    pub fn filter(mut self, f: NetworkFilter) -> Self { self.args.filter(f); self }
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NetworkPrune);

// ═══════════════════════════ VOLUMES ════════════════════════════════════════

pub struct VolumeHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> VolumeHandle<'a> {
    pub fn list(&self)                           -> VolumeQuery<'_>  { VolumeQuery::new(self.0) }
    pub fn create(&self, name: impl Into<String>) -> VolumeCreate<'_> { VolumeCreate::new(self.0, name) }
    pub fn prune(&self)                          -> VolumePrune<'_>  { VolumePrune::new(self.0) }
}

pub struct VolumeQuery<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> VolumeQuery<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["volume", "ls", "--format", "{{json .}}"]) }
    }
    pub fn filter(mut self, f: VolumeFilter) -> Self { self.args.filter(f); self }
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn list(self) -> DockerResult<Vec<VolumeSummary>> {
        self.cli.execute_json_lines(&self.args).await
    }
    pub async fn exists(self) -> DockerResult<bool> { Ok(!self.list().await?.is_empty()) }
}
crate::impl_builder_opts!(VolumeQuery);

pub struct VolumeCreate<'a> { cli: &'a DockerCli, name: String, args: ArgBuilder }

impl<'a> VolumeCreate<'a> {
    fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self { cli, name: name.into(), args: ArgBuilder::default() }
    }
    pub fn driver(mut self, v: impl Into<String>) -> Self { self.args.pair("--driver", v.into()); self }
    pub fn label(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.label(k, v); self }
    pub fn opt(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.pair("--opt", format!("{}={}", k.as_ref(), v.as_ref())); self
    }
    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["volume", "create"]);
        a.push_all(self.args.clone().build());
        a.push(&self.name);
        a.preview()
    }
    pub async fn run(self) -> DockerResult<String> {
        let mut a = ArgBuilder::cmd(&["volume", "create"]);
        a.push_all(self.args.build());
        a.push(&self.name);
        Ok(self.cli.execute(&a).await?.stdout.trim().to_string())
    }
}
crate::impl_builder_opts!(VolumeCreate);

pub struct VolumePrune<'a> { cli: &'a DockerCli, args: ArgBuilder }

impl<'a> VolumePrune<'a> {
    fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["volume", "prune", "--force"]) }
    }
    pub fn filter(mut self, f: VolumeFilter) -> Self { self.args.filter(f); self }
    pub fn all(mut self) -> Self { self.args.flag("--all"); self }
    pub fn print(&self) -> String { self.args.preview() }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(VolumePrune);
