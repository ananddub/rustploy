use super::{ExecError, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent};
use std::{ffi::OsStr, process::Stdio};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
    sync::mpsc,
};

#[derive(Clone, Debug, Default)]
pub struct LocalExecutor;
impl LocalExecutor {
    pub fn new() -> Self {
        Self
    }
    pub fn command<I, S>(&self, program: &str, args: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut c = Command::new(program);
        c.args(args);
        c
    }
    pub async fn run<I, S>(&self, program: &str, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        checked(self.command(program, args).output().await?)
    }
    pub async fn run_with_stdin<I, S>(
        &self,
        program: &str,
        args: I,
        stdin: impl AsRef<[u8]>,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut c = self.command(program, args);
        c.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = c.spawn()?;
        child
            .stdin
            .take()
            .expect("piped stdin")
            .write_all(stdin.as_ref())
            .await?;
        checked(child.wait_with_output().await?)
    }
    pub async fn run_stream(
        &self,
        program: &str,
        args: &[String],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut c = self.command(program, args);
        c.stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = c.spawn()?;
        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();
        let tx = sender.clone();
        let out = tokio::spawn(async move {
            let mut b = vec![0; 8192];
            loop {
                let n = stdout.read(&mut b).await?;
                if n == 0 {
                    break;
                }
                if tx
                    .send(ExecStreamEvent::Stdout(b[..n].to_vec()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Ok::<_, std::io::Error>(())
        });
        let tx = sender.clone();
        let err = tokio::spawn(async move {
            let mut b = vec![0; 8192];
            loop {
                let n = stderr.read(&mut b).await?;
                if n == 0 {
                    break;
                }
                if tx
                    .send(ExecStreamEvent::Stderr(b[..n].to_vec()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Ok::<_, std::io::Error>(())
        });
        let status = tokio::select! {s=child.wait()=>s?,_=sender.closed()=>{child.kill().await?;let _=child.wait().await;return Err(ExecError::StreamCancelled);}};
        out.await
            .map_err(|e| ExecError::Io(std::io::Error::other(e)))??;
        err.await
            .map_err(|e| ExecError::Io(std::io::Error::other(e)))??;
        let status = ExecExitStatus::Local(status);
        if !status.success() {
            return Err(ExecError::CommandFailed {
                code: status.code(),
                stderr: "streamed command failed".into(),
            });
        }
        Ok(status)
    }
}
fn checked(output: std::process::Output) -> ExecResult<ExecOutput> {
    let result = ExecOutput {
        status: ExecExitStatus::Local(output.status),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    };
    if !result.success() {
        return Err(ExecError::CommandFailed {
            code: result.status.code(),
            stderr: result.stderr.trim().into(),
        });
    }
    Ok(result)
}
