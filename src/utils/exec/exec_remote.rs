use super::{
    ExecError, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent, SshAuth, SshHostKey,
};
use crate::utils::session::{SshSessionLease, SshSessionPool};
use russh_extra::{Client, HostKeyPolicy, Identity, KeyboardInteractiveReply};
use std::time::Duration;
use std::{ffi::OsStr, sync::Arc};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RemoteExecutor {
    host: String,
    port: u16,
    username: String,
    auth: SshAuth,
    host_key: SshHostKey,
    sudo_password: Option<String>,
    pool: Arc<SshSessionPool>,
    command_timeout: Duration,
    connect_timeout: Duration,
    job_pid_file: Option<String>,
}

#[derive(Debug)]
pub struct RemoteTerminal {
    pub input: mpsc::Sender<Vec<u8>>,
    pub resize: mpsc::Sender<(u16, u16)>,
    pub cancel: CancellationToken,
    task: JoinHandle<ExecResult<()>>,
}

impl RemoteTerminal {
    pub fn cancel(&self) {
        self.cancel.cancel();
    }

    pub async fn wait(self) -> ExecResult<()> {
        match self.task.await {
            Ok(result) => result,
            Err(error) => Err(ExecError::Ssh(error.to_string())),
        }
    }
}

impl RemoteExecutor {
    pub fn new(
        host: impl Into<String>,
        port: u16,
        username: impl Into<String>,
        auth: SshAuth,
        host_key: SshHostKey,
    ) -> Self {
        Self {
            host: host.into(),
            port,
            username: username.into(),
            auth,
            host_key,
            sudo_password: None,
            pool: SshSessionPool::new(4),
            command_timeout: Duration::from_secs(300),
            connect_timeout: Duration::from_secs(15),
            job_pid_file: None,
        }
    }
    pub fn with_sudo(mut self) -> Self {
        self.sudo_password = match &self.auth {
            SshAuth::Password(p) => Some(p.clone()),
            _ => Some(String::new()),
        };
        self
    }
    pub fn with_sudo_password(mut self, password: impl Into<String>) -> Self {
        self.sudo_password = Some(password.into());
        self
    }
    pub fn with_pool_size(mut self, max_size: usize) -> Self {
        self.pool = SshSessionPool::new(max_size);
        self
    }
    pub fn with_pool_size_and_channels(
        mut self,
        max_size: usize,
        max_channels_per_session: usize,
    ) -> Self {
        self.pool = SshSessionPool::new_with_channels(max_size, max_channels_per_session);
        self
    }
    pub fn with_session_pool(mut self, pool: Arc<SshSessionPool>) -> Self {
        self.pool = pool;
        self
    }
    pub fn session_pool(&self) -> Arc<SshSessionPool> {
        self.pool.clone()
    }
    pub fn with_command_timeout(mut self, timeout: Duration) -> Self {
        self.command_timeout = timeout;
        self
    }
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }
    pub fn with_job_pid_file(mut self, pid_file: impl Into<String>) -> Self {
        self.job_pid_file = Some(pid_file.into());
        self
    }

    pub async fn open_terminal(
        &self,
        output: mpsc::Sender<ExecStreamEvent>,
        term: impl Into<String>,
        cols: u16,
        rows: u16,
    ) -> ExecResult<RemoteTerminal> {
        let session = self.connect_session().await?;
        let guard = session
            .russh_handle()
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;
        let mut channel = guard
            .channel_open_session()
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;

        channel
            .request_pty(true, &term.into(), cols.into(), rows.into(), 0, 0, &[])
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;
        channel
            .request_shell(true)
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;

        let (input_tx, mut input_rx) = mpsc::channel::<Vec<u8>>(128);
        let (resize_tx, mut resize_rx) = mpsc::channel::<(u16, u16)>(16);
        let cancel = CancellationToken::new();
        let task_cancel = cancel.clone();

        let task = tokio::spawn(async move {
            use russh_extra::russh::ChannelMsg;

            loop {
                tokio::select! {
                    _ = task_cancel.cancelled() => {
                        let _ = channel.close().await;
                        return Ok(());
                    }
                    input = input_rx.recv() => {
                        let Some(input) = input else {
                            let _ = channel.close().await;
                            return Ok(());
                        };
                        channel
                            .data(input.as_slice())
                            .await
                            .map_err(|e| ExecError::Ssh(e.to_string()))?;
                    }
                    resize = resize_rx.recv() => {
                        let Some((cols, rows)) = resize else {
                            continue;
                        };
                        channel
                            .window_change(cols.into(), rows.into(), 0, 0)
                            .await
                            .map_err(|e| ExecError::Ssh(e.to_string()))?;
                    }
                    message = channel.wait() => {
                        let Some(message) = message else {
                            return Ok(());
                        };

                        match message {
                            ChannelMsg::Data { data } => {
                                if output.send(ExecStreamEvent::Stdout(data.to_vec())).await.is_err() {
                                    let _ = channel.close().await;
                                    return Err(ExecError::StreamCancelled);
                                }
                            }
                            ChannelMsg::ExtendedData { data, .. } => {
                                if output.send(ExecStreamEvent::Stderr(data.to_vec())).await.is_err() {
                                    let _ = channel.close().await;
                                    return Err(ExecError::StreamCancelled);
                                }
                            }
                            ChannelMsg::ExitStatus { .. } | ChannelMsg::Close => {
                                return Ok(());
                            }
                            ChannelMsg::ExitSignal { .. } => {
                                let _ = channel.close().await;
                                return Err(ExecError::Ssh("remote terminal terminated by a signal".into()));
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Ok(RemoteTerminal {
            input: input_tx,
            resize: resize_tx,
            cancel,
            task,
        })
    }
    pub async fn kill_pid_file(&self, pid_file: impl AsRef<str>) -> ExecResult<()> {
        let command = remote_command(
            "sh",
            &["-c".into(), remote_cancel_script(pid_file.as_ref())],
            self.sudo_password.is_some(),
        );
        self.execute_raw_once(command, true, Duration::from_secs(8))
            .await
    }
    pub async fn run<I, S>(&self, program: &str, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = collect(args);
        self.execute(program, &args, &[], None, None).await
    }
    pub async fn run_cancelled<I, S>(
        &self,
        program: &str,
        args: I,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = collect(args);
        self.execute(program, &args, &[], None, Some(cancel)).await
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
        let args = collect(args);
        self.execute(program, &args, stdin.as_ref(), None, None)
            .await
    }
    pub async fn run_with_stdin_cancelled<I, S>(
        &self,
        program: &str,
        args: I,
        stdin: impl AsRef<[u8]>,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = collect(args);
        self.execute(program, &args, stdin.as_ref(), None, Some(cancel))
            .await
    }
    pub async fn run_stream(
        &self,
        program: &str,
        args: &[String],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        Ok(self
            .execute(program, args, &[], Some(sender), None)
            .await?
            .status)
    }
    async fn connect_session(&self) -> ExecResult<russh_extra::Session> {
        let mut builder = Client::builder()
            .endpoint((self.host.clone(), self.port))
            .username(self.username.clone());
        builder = match &self.auth {
            SshAuth::Password(password) => {
                let keyboard_password = password.clone();
                builder
                    .password(password.clone())
                    .keyboard_interactive(move |info| {
                        let password = keyboard_password.clone();
                        Box::pin(async move {
                            KeyboardInteractiveReply::Responses(
                                info.prompts().iter().map(|_| password.clone()).collect(),
                            )
                        })
                    })
            }
            SshAuth::KeyPair {
                private_key,
                passphrase,
                ..
            } => {
                let mut identity = Identity::load_openssh_pem(private_key.as_bytes().to_vec());
                if let Some(passphrase) = passphrase {
                    identity = identity.with_passphrase(passphrase.clone());
                }
                builder.identity(identity)
            }
            SshAuth::Agent => builder.agent(),
        };
        builder = match &self.host_key {
            SshHostKey::PinnedSha256(key) => builder.host_key_policy(
                HostKeyPolicy::pinned_sha256(key.clone())
                    .map_err(|e| ExecError::Ssh(e.to_string()))?,
            ),
            SshHostKey::InsecureAcceptAny => builder.accept_any_host_key(),
        };
        match tokio::time::timeout(self.connect_timeout, builder.build().connect()).await {
            Ok(result) => result.map_err(|e| ExecError::Ssh(e.to_string())),
            Err(_) => Err(ExecError::Timeout {
                seconds: self.connect_timeout.as_secs(),
            }),
        }
    }
    async fn execute(
        &self,
        program: &str,
        args: &[String],
        stdin: &[u8],
        stream: Option<mpsc::Sender<ExecStreamEvent>>,
        cancel: Option<&CancellationToken>,
    ) -> ExecResult<ExecOutput> {
        let mut last_error = None;
        for _ in 0..2 {
            let mut lease = self.pool.acquire().await?;
            if let SshSessionLease::New { connection_permit } = lease {
                lease = match self.connect_session().await {
                    Ok(session) => self.pool.attach(session, connection_permit).await?,
                    Err(error) => return Err(error),
                };
            }
            let Some(session) = lease.session() else {
                return Err(ExecError::Ssh("SSH session lease was not attached".into()));
            };
            let result = match tokio::time::timeout(
                self.command_timeout,
                self.execute_on_session(session, program, args, stdin, stream.clone(), cancel),
            )
            .await
            {
                Ok(result) => result,
                Err(_) => {
                    if cancel.is_some() {
                        if let Some(pid_file) = &self.job_pid_file {
                            if let Err(error) = self.kill_pid_file(pid_file).await {
                                tracing::warn!(
                                    error = %error,
                                    pid_file = %pid_file,
                                    "failed to kill timed out remote cancellable SSH job"
                                );
                            }
                        }
                    }
                    Err(ExecError::Timeout {
                        seconds: self.command_timeout.as_secs(),
                    })
                }
            };
            match result {
                Ok(output) => {
                    drop(lease);
                    return Ok(output);
                }
                Err(error @ ExecError::Ssh(_)) => {
                    last_error = Some(error);
                    self.pool.discard(&lease).await;
                    drop(lease);
                }
                Err(error) => {
                    if matches!(error, ExecError::Timeout { .. }) {
                        self.pool.discard(&lease).await;
                    }
                    drop(lease);
                    return Err(error);
                }
            }
        }
        Err(last_error
            .unwrap_or_else(|| ExecError::Ssh("SSH execution failed after reconnect".into())))
    }
    async fn execute_on_session(
        &self,
        session: &russh_extra::Session,
        program: &str,
        args: &[String],
        stdin: &[u8],
        stream: Option<mpsc::Sender<ExecStreamEvent>>,
        cancel: Option<&CancellationToken>,
    ) -> ExecResult<ExecOutput> {
        use russh_extra::russh::ChannelMsg;
        let guard = session
            .russh_handle()
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;
        let mut channel = guard
            .channel_open_session()
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;
        let base_command = remote_command(program, args, self.sudo_password.is_some());
        let cancel_job = cancel.map(|_| {
            self.job_pid_file
                .as_ref()
                .map(|pid_file| RemoteCancelJob::from_pid_file(pid_file.clone()))
                .unwrap_or_else(RemoteCancelJob::new)
        });
        let command = if let Some(job) = &cancel_job {
            cancellable_remote_command(&base_command, &job.pid_file)
        } else {
            base_command
        };
        channel
            .exec(true, command.into_bytes())
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;
        let mut input = Vec::new();
        if let Some(password) = &self.sudo_password {
            input.extend_from_slice(password.as_bytes());
            input.push(b'\n');
        }
        input.extend_from_slice(stdin);
        if !input.is_empty() {
            channel
                .data(input.as_slice())
                .await
                .map_err(|e| ExecError::Ssh(e.to_string()))?;
        }
        channel
            .eof()
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut exit = None;
        loop {
            let message = if let Some(cancel) = cancel {
                if let Some(sender) = &stream {
                    tokio::select! {
                        message=channel.wait()=>message,
                        _=sender.closed()=>{
                            self.cancel_remote_job(cancel_job.as_ref()).await;
                            let _=channel.close().await;
                            return Err(ExecError::StreamCancelled);
                        },
                        _=cancel.cancelled()=>{
                            self.cancel_remote_job(cancel_job.as_ref()).await;
                            let _=channel.close().await;
                            return Err(ExecError::StreamCancelled);
                        }
                    }
                } else {
                    tokio::select! {
                        message=channel.wait()=>message,
                        _=cancel.cancelled()=>{
                            self.cancel_remote_job(cancel_job.as_ref()).await;
                            let _=channel.close().await;
                            return Err(ExecError::StreamCancelled);
                        }
                    }
                }
            } else if let Some(sender) = &stream {
                tokio::select! {message=channel.wait()=>message,_=sender.closed()=>{let _=channel.close().await;return Err(ExecError::StreamCancelled);}}
            } else {
                channel.wait().await
            };
            let Some(message) = message else {
                break;
            };
            match message {
                ChannelMsg::Data { data } => {
                    if let Some(tx) = &stream {
                        if tx
                            .send(ExecStreamEvent::Stdout(data.to_vec()))
                            .await
                            .is_err()
                        {
                            let _ = channel.close().await;
                            return Err(ExecError::StreamCancelled);
                        }
                    } else {
                        stdout.extend_from_slice(&data)
                    }
                }
                ChannelMsg::ExtendedData { data, .. } => {
                    if let Some(tx) = &stream {
                        if tx
                            .send(ExecStreamEvent::Stderr(data.to_vec()))
                            .await
                            .is_err()
                        {
                            let _ = channel.close().await;
                            return Err(ExecError::StreamCancelled);
                        }
                    } else {
                        stderr.extend_from_slice(&data)
                    }
                }
                ChannelMsg::ExitStatus { exit_status } => {
                    exit = Some(exit_status);
                    break;
                }
                ChannelMsg::ExitSignal { .. } => {
                    let _ = channel.close().await;
                    return Err(ExecError::Ssh(
                        "remote command terminated by a signal".into(),
                    ));
                }
                ChannelMsg::Close => break,
                _ => {}
            }
        }
        let _ = channel.close().await;
        let status =
            ExecExitStatus::Remote(exit.ok_or_else(|| {
                ExecError::Ssh("remote command ended without an exit status".into())
            })?);
        let result = ExecOutput {
            status,
            stdout: String::from_utf8_lossy(&stdout).into_owned(),
            stderr: String::from_utf8_lossy(&stderr).into_owned(),
        };
        if !result.success() {
            return Err(ExecError::CommandFailed {
                code: result.status.code(),
                stderr: result.combined_output(),
            });
        }
        Ok(result)
    }

    async fn cancel_remote_job(&self, job: Option<&RemoteCancelJob>) {
        let Some(job) = job else {
            return;
        };
        if let Err(error) = self.kill_pid_file(&job.pid_file).await {
            tracing::warn!(
                error = %error,
                pid_file = %job.pid_file,
                "failed to kill remote cancellable SSH job"
            );
        }
    }

    async fn execute_raw_once(
        &self,
        command: String,
        send_sudo_password: bool,
        timeout: Duration,
    ) -> ExecResult<()> {
        let session = self.connect_session().await?;
        let result = tokio::time::timeout(timeout, async {
            use russh_extra::russh::ChannelMsg;

            let guard = session
                .russh_handle()
                .await
                .map_err(|e| ExecError::Ssh(e.to_string()))?;
            let mut channel = guard
                .channel_open_session()
                .await
                .map_err(|e| ExecError::Ssh(e.to_string()))?;
            channel
                .exec(true, command.into_bytes())
                .await
                .map_err(|e| ExecError::Ssh(e.to_string()))?;

            if send_sudo_password {
                if let Some(password) = &self.sudo_password {
                    let mut input = password.as_bytes().to_vec();
                    input.push(b'\n');
                    channel
                        .data(input.as_slice())
                        .await
                        .map_err(|e| ExecError::Ssh(e.to_string()))?;
                }
            }
            channel
                .eof()
                .await
                .map_err(|e| ExecError::Ssh(e.to_string()))?;

            let mut exit = None;
            while let Some(message) = channel.wait().await {
                match message {
                    ChannelMsg::ExitStatus { exit_status } => {
                        exit = Some(exit_status);
                        break;
                    }
                    ChannelMsg::ExitSignal { .. } => {
                        let _ = channel.close().await;
                        return Err(ExecError::Ssh(
                            "remote cancel command terminated by a signal".into(),
                        ));
                    }
                    ChannelMsg::Close => break,
                    _ => {}
                }
            }
            let _ = channel.close().await;
            match exit {
                Some(0) => Ok(()),
                Some(code) => Err(ExecError::CommandFailed {
                    code: Some(code as i32),
                    stderr: "remote cancel command failed".into(),
                }),
                None => Err(ExecError::Ssh(
                    "remote cancel command ended without an exit status".into(),
                )),
            }
        })
        .await;

        match result {
            Ok(result) => result,
            Err(_) => Err(ExecError::Timeout {
                seconds: timeout.as_secs(),
            }),
        }
    }
}

#[derive(Clone, Debug)]
struct RemoteCancelJob {
    pid_file: String,
}
impl RemoteCancelJob {
    fn new() -> Self {
        Self {
            pid_file: format!("/tmp/rustploy-ssh-job-{}.pid", Uuid::new_v4()),
        }
    }

    fn from_pid_file(pid_file: String) -> Self {
        Self { pid_file }
    }
}

fn collect<I, S>(args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    args.into_iter()
        .map(|v| v.as_ref().to_string_lossy().into_owned())
        .collect()
}
fn quote(value: &str) -> String {
    if value.is_empty() {
        return "''".into();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}
fn remote_command(program: &str, args: &[String], sudo: bool) -> String {
    let prefix = if sudo {
        vec![
            "sudo".into(),
            "-S".into(),
            "-p".into(),
            String::new(),
            "--".into(),
            program.into(),
        ]
    } else {
        vec![program.into()]
    };
    prefix
        .into_iter()
        .chain(args.iter().cloned())
        .map(|v| quote(&v))
        .collect::<Vec<_>>()
        .join(" ")
}

fn cancellable_remote_command(command: &str, pid_file: &str) -> String {
    let script = format!(
        r#"rm -f {pid_file}
setsid sh -c {command} &
child="$!"
printf '%s\n' "$child" > {pid_file}
wait "$child"
status="$?"
rm -f {pid_file}
exit "$status""#,
        pid_file = quote(pid_file),
        command = quote(command),
    );
    remote_command("sh", &["-c".into(), script], false)
}

fn remote_cancel_script(pid_file: &str) -> String {
    format!(
        r#"pid="$(cat {pid_file} 2>/dev/null || true)"
if [ -n "$pid" ]; then
  kill -TERM -- "-$pid" 2>/dev/null || true
  sleep 2
  kill -KILL -- "-$pid" 2>/dev/null || true
fi
rm -f {pid_file}"#,
        pid_file = quote(pid_file),
    )
}
