#!/bin/bash
echo '
// ── Generated Simple Builders ───────────────────────────────────────────────'

for cmd in start stop restart pause unpause kill wait port top; do
  struct="Container$(tr '[:lower:]' '[:upper:]' <<< ${cmd:0:1})${cmd:1}Builder"
  cat << INNER
pub struct $struct<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> $struct<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "$cmd"]) }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!($struct);

INNER
done

cat << 'INNER2'
pub struct ContainerRmBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerRmBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "rm"]) }
    }
    pub fn force(mut self) -> Self { self.args.flag("--force"); self }
    pub fn volumes(mut self) -> Self { self.args.flag("--volumes"); self }
    pub fn link(mut self) -> Self { self.args.flag("--link"); self }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerRmBuilder);

pub struct ContainerRenameBuilder<'a> { cli: &'a DockerCli, id: String, name: String, args: ArgBuilder }
impl<'a> ContainerRenameBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>, name: impl Into<String>) -> Self {
        Self { cli, id: id.into(), name: name.into(), args: ArgBuilder::cmd(&["container", "rename"]) }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        a.push(&self.name);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerRenameBuilder);

pub struct ContainerUpdateBuilder<'a> { cli: &'a DockerCli, id: String, args: ArgBuilder }
impl<'a> ContainerUpdateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self { cli, id: id.into(), args: ArgBuilder::cmd(&["container", "update"]) }
    }
    pub fn memory(mut self, m: crate::utils::docker::core::Memory) -> Self { self.args.pair("--memory", m.to_string()); self }
    pub fn cpus(mut self, c: crate::utils::docker::core::Cpu) -> Self { self.args.pair("--cpus", c.to_string()); self }
    pub fn restart(mut self, p: crate::utils::docker::RestartPolicy) -> Self { self.args.pair("--restart", p.to_string()); self }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerUpdateBuilder);
INNER2

