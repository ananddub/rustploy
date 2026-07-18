use crate::utils::{
    docker::{
        core::ArgBuilder,
        client::DockerCli,
        DockerOutput, DockerResult,
    },
};

pub struct SecretRemoveBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> SecretRemoveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["secret", "rm"]), name: name.into() }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SecretRemoveBuilder);
