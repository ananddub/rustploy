use super::{DockerError, DockerOutput, DockerResult};
use serde::de::DeserializeOwned;
use std::{ffi::OsStr, path::PathBuf, process::Stdio};
use tokio::{io::AsyncWriteExt, process::Command};

#[derive(Clone, Debug)]
pub struct DockerCli {
    executable: PathBuf,
    global_args: Vec<String>,
}

impl Default for DockerCli {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerCli {
    pub fn new() -> Self {
        Self {
            executable: "docker".into(),
            global_args: Vec::new(),
        }
    }
    pub fn with_executable(executable: impl Into<PathBuf>) -> Self {
        Self {
            executable: executable.into(),
            global_args: Vec::new(),
        }
    }
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.global_args.extend(["--host".into(), host.into()]);
        self
    }
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.global_args
            .extend(["--context".into(), context.into()]);
        self
    }
    pub fn command<I, S>(&self, args: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new(&self.executable);
        command.args(&self.global_args).args(args);
        command
    }
    pub async fn run<I, S>(&self, args: I) -> DockerResult<DockerOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = self.command(args).output().await?;
        Self::checked_output(output)
    }
    pub async fn run_with_stdin<I, S>(
        &self,
        args: I,
        stdin: impl AsRef<[u8]>,
    ) -> DockerResult<DockerOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = self.command(args);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn()?;
        child
            .stdin
            .take()
            .expect("piped stdin")
            .write_all(stdin.as_ref())
            .await?;
        Self::checked_output(child.wait_with_output().await?)
    }
    pub(crate) async fn json<T: DeserializeOwned>(&self, args: &[&str]) -> DockerResult<T> {
        Ok(serde_json::from_str(&self.run(args).await?.stdout)?)
    }
    pub(crate) async fn json_lines<T: DeserializeOwned>(
        &self,
        args: &[&str],
    ) -> DockerResult<Vec<T>> {
        self.run(args)
            .await?
            .stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).map_err(Into::into))
            .collect()
    }
    pub(crate) async fn prefixed(
        &self,
        prefix: &[&str],
        args: &[&str],
    ) -> DockerResult<DockerOutput> {
        let mut command = prefix.to_vec();
        command.extend_from_slice(args);
        self.run(command).await
    }
    pub(crate) async fn list<T: DeserializeOwned>(
        &self,
        object: &str,
        filters: &[&str],
    ) -> DockerResult<Vec<T>> {
        let mut args = vec![object, "ls", "--format", "{{json .}}"];
        for filter in filters {
            args.extend(["--filter", filter]);
        }
        self.json_lines(&args).await
    }
    pub(crate) async fn prune(&self, object: &str, filters: &[&str]) -> DockerResult<DockerOutput> {
        let mut args = vec![object, "prune", "--force"];
        for filter in filters {
            args.extend(["--filter", filter]);
        }
        self.run(args).await
    }
    fn checked_output(output: std::process::Output) -> DockerResult<DockerOutput> {
        let result = DockerOutput {
            status: output.status,
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        };
        if !result.success() {
            return Err(DockerError::CommandFailed {
                code: result.status.code(),
                stderr: result.stderr.trim().into(),
            });
        }
        Ok(result)
    }
}
