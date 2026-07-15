use crate::utils::{
    exec::{ArgBuilder, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent},
    git::{client::GitCli, GitBranch},
};
use tokio_util::sync::CancellationToken;

// ── LsRemoteBuilder ──────────────────────────────────────────────────────────

pub struct LsRemoteBuilder<'a> {
    cli: &'a GitCli,
    repository: String,
    args: ArgBuilder,
}

impl<'a> LsRemoteBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli, repository: impl Into<String>) -> Self {
        Self {
            cli,
            repository: repository.into(),
            args: ArgBuilder::cmd(&["ls-remote"]),
        }
    }

    /// Limit to only refs/heads
    pub fn heads(mut self) -> Self { self.args.flag("--heads"); self }
    /// Limit to only refs/tags
    pub fn tags(mut self) -> Self { self.args.flag("--tags"); self }
    /// Show the underlying ref instead of object id for symrefs
    pub fn symref(mut self) -> Self { self.args.flag("--symref"); self }
    /// Only show refs that match the pattern
    pub fn ref_pattern(mut self, pattern: impl Into<String>) -> Self { self.args.push(pattern.into()); self }

    pub fn print(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.repository);
        a.preview()
    }

    pub async fn output(self) -> ExecResult<ExecOutput> {
        let mut a = self.args;
        a.push(&self.repository);
        let built = a.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run(refs).await
    }
    
    pub async fn output_cancelled(self, cancel: &CancellationToken) -> ExecResult<ExecOutput> {
        let mut a = self.args;
        a.push(&self.repository);
        let built = a.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run_cancelled(refs, cancel).await
    }
}

// ── GitQueries (High level wrappers) ──────────────────────────────────────────

pub struct GitQueries<'a>(pub(crate) &'a GitCli);

impl<'a> GitQueries<'a> {
    pub fn ls_remote(&self, repository: impl Into<String>) -> LsRemoteBuilder<'_> {
        LsRemoteBuilder::new(self.0, repository)
    }

    /// Retrieve all remote branches (heads) without cloning.
    pub async fn remote_branches(&self, repository_url: &str) -> ExecResult<Vec<GitBranch>> {
        let out = self.ls_remote(repository_url).heads().output().await?;
        parse_remote_branches(&out.stdout)
    }

    /// Retrieve the default branch name (usually `main` or `master`) without cloning.
    pub async fn remote_default_branch(&self, repository_url: &str) -> ExecResult<Option<String>> {
        let out = self.ls_remote(repository_url).symref().ref_pattern("HEAD").output().await?;
        Ok(parse_remote_default_branch(&out.stdout))
    }

    pub async fn remote_default_branch_cancelled(
        &self,
        repository_url: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<Option<String>> {
        let out = self.ls_remote(repository_url).symref().ref_pattern("HEAD").output_cancelled(cancel).await?;
        Ok(parse_remote_default_branch(&out.stdout))
    }
}

// ── Parsing Helpers ──────────────────────────────────────────────────────────

fn parse_remote_default_branch(output: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let line = line.strip_prefix("ref: ")?;
        let (reference, target) = line.split_once(char::is_whitespace)?;
        if target.trim() != "HEAD" {
            return None;
        }
        reference
            .trim()
            .strip_prefix("refs/heads/")
            .map(str::to_owned)
    })
}

fn parse_remote_branches(output: &str) -> ExecResult<Vec<GitBranch>> {
    let mut branches = output
        .lines()
        .filter_map(|line| {
            let (_, reference) = line.split_once(char::is_whitespace)?;
            let name = reference.trim().strip_prefix("refs/heads/")?;
            Some(GitBranch {
                current: false,
                name: name.to_owned(),
            })
        })
        .collect::<Vec<_>>();
    branches.sort_by(|a, b| a.name.cmp(&b.name));
    branches.dedup_by(|a, b| a.name == b.name);
    Ok(branches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_remote_branch_names_from_ls_remote_heads() {
        let output = "\
1111111111111111111111111111111111111111\trefs/heads/main
2222222222222222222222222222222222222222\trefs/heads/feature/login
3333333333333333333333333333333333333333\trefs/tags/v1.0.0
";
        let branches = parse_remote_branches(output).unwrap();
        assert_eq!(
            branches,
            vec![
                GitBranch { name: "feature/login".into(), current: false },
                GitBranch { name: "main".into(), current: false },
            ]
        );
    }

    #[test]
    fn parses_remote_default_branch_from_symref_head() {
        let output = "\
ref: refs/heads/master\tHEAD
1111111111111111111111111111111111111111\tHEAD
";
        assert_eq!(parse_remote_default_branch(output), Some("master".into()));
    }
}
