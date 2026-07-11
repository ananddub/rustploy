use super::{GitBranch, GitStatusEntry};
use crate::utils::exec::{
    CommandExecutor, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent, LocalExecutor,
    RemoteExecutor, SshAuth, SshHostKey,
};
use std::{ffi::OsStr, path::PathBuf};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct GitCli {
    executor: CommandExecutor,
    executable: String,
    repository: Option<String>,
}

impl Default for GitCli {
    fn default() -> Self {
        Self::new_local()
    }
}

impl GitCli {
    pub fn new_local() -> Self {
        Self {
            executor: CommandExecutor::Local(LocalExecutor::new()),
            executable: "git".into(),
            repository: None,
        }
    }
    pub fn from_executor(executor: CommandExecutor) -> Self {
        Self {
            executor,
            executable: "git".into(),
            repository: None,
        }
    }
    pub fn new_remote(
        host: impl Into<String>,
        port: u16,
        username: impl Into<String>,
        auth: SshAuth,
        host_key: SshHostKey,
    ) -> Self {
        Self::from_remote_executor(RemoteExecutor::new(host, port, username, auth, host_key))
    }
    pub fn from_remote_executor(executor: RemoteExecutor) -> Self {
        Self {
            executor: CommandExecutor::Remote(executor),
            executable: "git".into(),
            repository: None,
        }
    }
    pub fn with_executable(mut self, executable: impl Into<PathBuf>) -> Self {
        self.executable = executable.into().to_string_lossy().into_owned();
        self
    }
    pub fn with_repository(mut self, path: impl Into<String>) -> Self {
        self.repository = Some(path.into());
        self
    }
    pub fn with_remote_sudo(mut self) -> Self {
        if let CommandExecutor::Remote(remote) = self.executor {
            self.executor = CommandExecutor::Remote(remote.with_sudo());
        }
        self
    }
    pub fn with_remote_sudo_password(mut self, password: impl Into<String>) -> Self {
        if let CommandExecutor::Remote(remote) = self.executor {
            self.executor = CommandExecutor::Remote(remote.with_sudo_password(password));
        }
        self
    }

    pub async fn run<I, S>(&self, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run(&self.executable, self.arguments(args))
            .await
    }
    pub async fn run_cancelled<I, S>(
        &self,
        args: I,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run_cancelled(&self.executable, self.arguments(args), cancel)
            .await
    }
    pub async fn run_with_stdin<I, S>(
        &self,
        args: I,
        stdin: impl AsRef<[u8]>,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run_with_stdin(&self.executable, self.arguments(args), stdin)
            .await
    }
    pub async fn run_stream<I, S>(
        &self,
        args: I,
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run_stream(&self.executable, self.arguments(args), sender)
            .await
    }
    fn arguments<I, S>(&self, args: I) -> Vec<String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut result = Vec::new();
        if let Some(repository) = &self.repository {
            result.extend(["-C".into(), repository.clone()]);
        }
        result.extend(
            args.into_iter()
                .map(|v| v.as_ref().to_string_lossy().into_owned()),
        );
        result
    }

    pub async fn init(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["init"], args).await
    }
    pub async fn clone_repository(
        &self,
        url: &str,
        destination: Option<&str>,
        options: &[&str],
    ) -> ExecResult<ExecOutput> {
        let mut args = vec!["clone"];
        args.extend_from_slice(options);
        args.push(url);
        if let Some(destination) = destination {
            args.push(destination);
        }
        self.run(args).await
    }
    pub async fn clone_repository_cancelled(
        &self,
        url: &str,
        destination: Option<&str>,
        options: &[&str],
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let mut args = vec!["clone"];
        args.extend_from_slice(options);
        args.push(url);
        if let Some(destination) = destination {
            args.push(destination);
        }
        self.run_cancelled(args, cancel).await
    }
    pub async fn clone_repository_stream(
        &self,
        url: &str,
        destination: Option<&str>,
        options: &[&str],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut args = vec!["clone"];
        args.extend_from_slice(options);
        args.push(url);
        if let Some(destination) = destination {
            args.push(destination);
        }
        self.run_stream(args, sender).await
    }
    pub async fn fetch(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["fetch"], args).await
    }
    pub async fn fetch_cancelled(
        &self,
        args: &[&str],
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let mut command = vec!["fetch"];
        command.extend_from_slice(args);
        self.run_cancelled(command, cancel).await
    }
    pub async fn fetch_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut command = vec!["fetch"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn pull(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["pull"], args).await
    }
    pub async fn pull_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut command = vec!["pull"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn push(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["push"], args).await
    }
    pub async fn push_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut command = vec!["push"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn checkout(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["checkout"], args).await
    }
    pub async fn switch(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["switch"], args).await
    }
    pub async fn add(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["add"], args).await
    }
    pub async fn commit(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["commit"], args).await
    }
    pub async fn merge(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["merge"], args).await
    }
    pub async fn rebase(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["rebase"], args).await
    }
    pub async fn reset(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["reset"], args).await
    }
    pub async fn restore(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["restore"], args).await
    }
    pub async fn clean(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["clean"], args).await
    }
    pub async fn tag(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["tag"], args).await
    }
    pub async fn remote(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["remote"], args).await
    }
    pub async fn config(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["config"], args).await
    }
    pub async fn submodule(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["submodule"], args).await
    }
    pub async fn worktree(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["worktree"], args).await
    }
    pub async fn stash(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["stash"], args).await
    }
    pub async fn log(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["log"], args).await
    }
    pub async fn diff(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["diff"], args).await
    }
    pub async fn show(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["show"], args).await
    }
    pub async fn rev_parse(&self, revision: &str) -> ExecResult<String> {
        Ok(self
            .run(["rev-parse", revision])
            .await?
            .stdout
            .trim()
            .into())
    }
    pub async fn ls_remote(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["ls-remote"], args).await
    }
    pub async fn status(&self) -> ExecResult<Vec<GitStatusEntry>> {
        let output = self.run(["status", "--porcelain=v1", "-z"]).await?;
        let mut records = output
            .stdout
            .split('\0')
            .filter(|record| !record.is_empty());
        let mut result = Vec::new();
        while let Some(record) = records.next() {
            if let Some(mut entry) = GitStatusEntry::parse(record) {
                if matches!(entry.index_status, 'R' | 'C')
                    || matches!(entry.worktree_status, 'R' | 'C')
                {
                    entry.original_path = records.next().map(str::to_owned);
                }
                result.push(entry);
            }
        }
        Ok(result)
    }
    pub async fn branches(&self) -> ExecResult<Vec<GitBranch>> {
        Ok(self
            .run(["branch", "--format=%(HEAD)%00%(refname:short)%00"])
            .await?
            .stdout
            .lines()
            .filter_map(|line| {
                let mut p = line.split('\0');
                Some(GitBranch {
                    current: p.next()? == "*",
                    name: p.next()?.into(),
                })
            })
            .collect())
    }
    async fn prefixed(&self, prefix: &[&str], args: &[&str]) -> ExecResult<ExecOutput> {
        let mut command = prefix.to_vec();
        command.extend_from_slice(args);
        self.run(command).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn local_repository_status_commit_and_revision_are_typed() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_string_lossy().into_owned();
        let git = GitCli::new_local().with_repository(path);
        git.init(&[]).await.unwrap();
        git.config(&["user.name", "Rustploy Test"]).await.unwrap();
        git.config(&["user.email", "test@rustploy.local"])
            .await
            .unwrap();
        std::fs::write(dir.path().join("hello.txt"), "hello").unwrap();
        let status = git.status().await.unwrap();
        assert_eq!(status.len(), 1);
        assert_eq!(status[0].path, "hello.txt");
        assert_eq!(status[0].index_status, '?');
        git.add(&["hello.txt"]).await.unwrap();
        git.commit(&["-m", "initial"]).await.unwrap();
        let revision = git.rev_parse("HEAD").await.unwrap();
        assert_eq!(revision.len(), 40);
        assert!(git.status().await.unwrap().is_empty());
    }
}
