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
    pub fn rm(&self, name: impl Into<String>)   -> NetworkRmBuilder<'_> { NetworkRmBuilder::new(self.0, name) }
    pub fn connect(&self, network: impl Into<String>, container: impl Into<String>) -> NetworkConnectBuilder<'_> { NetworkConnectBuilder::new(self.0, network, container) }
    pub fn disconnect(&self, network: impl Into<String>, container: impl Into<String>) -> NetworkDisconnectBuilder<'_> { NetworkDisconnectBuilder::new(self.0, network, container) }
    pub async fn inspect(&self, name: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.0.run(["network", "inspect", name.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
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
    pub fn driver(mut self, d: crate::utils::docker::NetworkDriver)  -> Self {
        self.args.pair("--driver", d.as_str().to_string());
        self
    }
    pub fn subnet(mut self, v: impl Into<String>)  -> Self { self.args.pair("--subnet", v.into()); self }
    pub fn gateway(mut self, v: impl Into<String>) -> Self { self.args.pair("--gateway", v.into()); self }
    pub fn label(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self { self.args.label(k, v); self }
    pub fn opt(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.pair("--opt", format!("{}={}", k.as_ref(), v.as_ref())); self
    }
    pub fn attachable(mut self) -> Self { self.args.flag("--attachable"); self }
    pub fn internal(mut self)   -> Self { self.args.flag("--internal"); self }
    pub fn ipv6(mut self)       -> Self { self.args.flag("--ipv6"); self }
    pub fn scope(mut self, s: crate::utils::docker::NetworkScope) -> Self {
        self.args.pair("--scope", s.to_string());
        self
    }
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
    pub fn rm(&self, name: impl Into<String>)    -> VolumeRmBuilder<'_> { VolumeRmBuilder::new(self.0, name) }
    pub async fn inspect(&self, name: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.0.run(["volume", "inspect", name.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
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
    pub fn driver(mut self, d: crate::utils::docker::VolumeDriver) -> Self {
        self.args.pair("--driver", d.as_str().to_string());
        self
    }
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

// ── Generated Simple Builders ───────────────────────────────────────────────

pub struct NetworkRmBuilder<'a> { cli: &'a DockerCli, name: String, args: ArgBuilder }
impl<'a> NetworkRmBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self { Self { cli, name: name.into(), args: ArgBuilder::cmd(&["network", "rm"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.name); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(NetworkRmBuilder);

pub struct NetworkConnectBuilder<'a> { cli: &'a DockerCli, network: String, container: String, args: ArgBuilder }
impl<'a> NetworkConnectBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, network: impl Into<String>, container: impl Into<String>) -> Self { Self { cli, network: network.into(), container: container.into(), args: ArgBuilder::cmd(&["network", "connect"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.network); a.push(&self.container); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(NetworkConnectBuilder);

pub struct NetworkDisconnectBuilder<'a> { cli: &'a DockerCli, network: String, container: String, args: ArgBuilder }
impl<'a> NetworkDisconnectBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, network: impl Into<String>, container: impl Into<String>) -> Self { Self { cli, network: network.into(), container: container.into(), args: ArgBuilder::cmd(&["network", "disconnect"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.network); a.push(&self.container); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(NetworkDisconnectBuilder);

pub struct VolumeRmBuilder<'a> { cli: &'a DockerCli, name: String, args: ArgBuilder }
impl<'a> VolumeRmBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self { Self { cli, name: name.into(), args: ArgBuilder::cmd(&["volume", "rm"]) } }
    pub async fn run(self) -> DockerResult<DockerOutput> { let mut a = self.args; a.push(&self.name); self.cli.execute(&a).await }
}
crate::impl_builder_opts!(VolumeRmBuilder);

