use std::process::ExitStatus;
use thiserror::Error;

#[derive(Debug)]
pub struct DockerOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

impl DockerOutput {
    pub fn success(&self) -> bool {
        self.status.success()
    }
    pub fn stdout_trimmed(&self) -> &str {
        self.stdout.trim()
    }
}

#[derive(Debug, Error)]
pub enum DockerError {
    #[error("failed to execute Docker CLI: {0}")]
    Io(#[from] std::io::Error),
    #[error("Docker command failed (exit code {code:?}): {stderr}")]
    CommandFailed { code: Option<i32>, stderr: String },
    #[error("invalid JSON returned by Docker CLI: {0}")]
    Json(#[from] serde_json::Error),
}

pub type DockerResult<T> = Result<T, DockerError>;
