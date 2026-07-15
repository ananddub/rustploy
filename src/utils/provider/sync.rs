use crate::utils::{
    builder::{shared::BuilderContext, spec::BuilderEvent},
    exec::{CommandExecutor, ExecResult, LocalExecutor},
    git::{client::GitCli, types::GitAuth},
};
use std::path::Path;
use tokio_util::sync::CancellationToken;

/// High-level orchestration builder for cloning or fetching Git repositories.
pub struct ProviderSyncBuilder<'a> {
    url: String,
    destination: &'a str,
    branch: Option<&'a str>,
    executor: Option<CommandExecutor>,
    auth: Option<GitAuth>,
    submodules: bool,
    cancel: Option<&'a CancellationToken>,
    // Optional ctx just for emitting events if available
    ctx: Option<&'a BuilderContext>,
}

impl<'a> ProviderSyncBuilder<'a> {
    pub fn new(url: impl Into<String>, destination: &'a str) -> Self {
        Self {
            url: url.into(),
            destination,
            branch: None,
            executor: None,
            auth: None,
            submodules: false,
            cancel: None,
            ctx: None,
        }
    }

    pub fn branch(mut self, branch: &'a str) -> Self {
        self.branch = Some(branch);
        self
    }

    pub fn executor(mut self, executor: CommandExecutor) -> Self {
        self.executor = Some(executor);
        self
    }

    pub fn auth(mut self, auth: GitAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn submodules(mut self, submodules: bool) -> Self {
        self.submodules = submodules;
        self
    }

    pub fn cancel_with(mut self, cancel: &'a CancellationToken) -> Self {
        self.cancel = Some(cancel);
        self
    }

    pub fn context(mut self, ctx: &'a BuilderContext) -> Self {
        self.ctx = Some(ctx);
        self.executor = Some(ctx.executor.clone());
        self
    }

    /// Executes the sync operation (fetch + reset if exists, clone if not).
    pub async fn run(self) -> ExecResult<()> {
        let executor = self
            .executor
            .unwrap_or_else(|| CommandExecutor::Local(LocalExecutor::new()));
        
        let git = GitCli::from_executor(executor.clone()).with_repository(self.destination);
        let git_dir = format!("{}/.git", self.destination);
        let branch = self.branch.unwrap_or("main");

        // Use the dummy cancellation token if none was provided
        let dummy_cancel = CancellationToken::new();
        let cancel = self.cancel.unwrap_or(&dummy_cancel);

        if executor.run("test", ["-d", &git_dir]).await.is_ok() {
            if let Some(ctx) = self.ctx {
                ctx.emit(BuilderEvent::Message(format!(
                    "fetching source {} branch {} into {}",
                    self.url, branch, self.destination
                )))
                .await;
            }

            git.remote().set_url("origin", &self.url).run().await?;
            
            let mut fetch = git.fetch().prune().remote("origin").arg(branch);
            if let Some(auth) = &self.auth {
                fetch = fetch.auth(auth.clone());
            }
            fetch.run_cancelled(cancel).await?;
            
            git.reset().hard().commit("FETCH_HEAD").run().await?;
        } else {
            if let Some(parent) = Path::new(self.destination).parent() {
                executor
                    .run_cancelled("mkdir", ["-p", parent.to_string_lossy().as_ref()], cancel)
                    .await?;
            }
            
            if let Some(ctx) = self.ctx {
                ctx.emit(BuilderEvent::Message(format!(
                    "cloning source {} branch {} into {}",
                    self.url, branch, self.destination
                )))
                .await;
            }

            let clone_git = GitCli::from_executor(executor.clone());
            let mut clone = clone_git
                .clone(&self.url)
                .destination(self.destination)
                .branch(branch)
                .single_branch();
                
            if let Some(auth) = &self.auth {
                clone = clone.auth(auth.clone());
            }
            
            clone.run_cancelled(cancel).await?;
        }

        if self.submodules {
            let mut sub = git.submodule().update().init().recursive();
            if let Some(auth) = &self.auth {
                sub = sub.auth(auth.clone());
            }
            sub.run().await?;
        }

        Ok(())
    }
}
