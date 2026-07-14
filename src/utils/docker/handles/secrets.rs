use crate::utils::{
    docker::{
        core::{types::DataSource, ArgBuilder},
        client::DockerCli,
        DockerOutput, DockerResult,
    },
};

// ── SecretsHandle ───────────────────────────────────────────────────────────

pub struct SecretsHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> SecretsHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn create(&self, name: impl Into<String>) -> SecretCreateBuilder<'a> {
        SecretCreateBuilder::new(self.cli, name)
    }

    pub fn remove(&self, name: impl Into<String>) -> SecretRemoveBuilder<'a> {
        SecretRemoveBuilder::new(self.cli, name)
    }

    pub fn list(&self) -> SecretListBuilder<'a> {
        SecretListBuilder::new(self.cli)
    }

    pub async fn inspect(&self, name: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.cli.run(["secret", "inspect", name.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

// ── SecretCreateBuilder ─────────────────────────────────────────────────────

pub struct SecretCreateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
    data_source: Option<DataSource>,
}

impl<'a> SecretCreateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["secret", "create"]), name: name.into(), data_source: None }
    }

    pub fn from_file(mut self, path: impl Into<std::path::PathBuf>) -> Self { self.data_source = Some(DataSource::from_file(path)); self }
    pub fn from_string(mut self, data: impl Into<String>) -> Self { self.data_source = Some(DataSource::from_string(data)); self }
    pub fn from_bytes(mut self, data: impl Into<Vec<u8>>) -> Self { self.data_source = Some(DataSource::from_bytes(data)); self }

    pub fn label(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self { self.args.pair("--label", format!("{}={}", key.as_ref(), value.as_ref())); self }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        let source = self.data_source.take().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Data source required for secret creation"))?;
        self.args.push(&self.name);
        
        match source {
            DataSource::File(path) => {
                self.args.push(path.to_string_lossy().to_string());
                self.cli.execute(&self.args).await
            }
            DataSource::String(data) => {
                self.args.push("-");
                self.cli.run_with_stdin(&self.args.build(), data.as_bytes()).await
            }
            DataSource::Bytes(data) => {
                self.args.push("-");
                self.cli.run_with_stdin(&self.args.build(), &data).await
            }
        }
    }
}
crate::impl_builder_opts!(SecretCreateBuilder);

// ── SecretRemoveBuilder ─────────────────────────────────────────────────────

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

// ── SecretListBuilder ───────────────────────────────────────────────────────

pub struct SecretListBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SecretListBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, args: ArgBuilder::cmd(&["secret", "ls"]) }
    }

    pub fn filter(mut self, f: crate::utils::docker::query::filter::SecretFilter) -> Self { self.args.filter(f); self }
    pub fn filters(mut self, fs: impl IntoIterator<Item = crate::utils::docker::query::filter::SecretFilter>) -> Self {
        for f in fs { self.args.filter(f); }
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::SecretSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(SecretListBuilder);
