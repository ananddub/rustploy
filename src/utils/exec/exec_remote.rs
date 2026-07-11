use super::{
    ExecError, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent, SshAuth, SshHostKey,
};
use russh_extra::{Client, HostKeyPolicy, Identity, KeyboardInteractiveReply};
use std::ffi::OsStr;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct RemoteExecutor {
    host: String,
    port: u16,
    username: String,
    auth: SshAuth,
    host_key: SshHostKey,
    sudo_password: Option<String>,
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
    pub async fn run<I, S>(&self, program: &str, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = collect(args);
        self.execute(program, &args, &[], None).await
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
        self.execute(program, &args, stdin.as_ref(), None).await
    }
    pub async fn run_stream(
        &self,
        program: &str,
        args: &[String],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        Ok(self.execute(program, args, &[], Some(sender)).await?.status)
    }
    async fn session(&self) -> ExecResult<russh_extra::Session> {
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
        builder
            .build()
            .connect()
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))
    }
    async fn execute(
        &self,
        program: &str,
        args: &[String],
        stdin: &[u8],
        stream: Option<mpsc::Sender<ExecStreamEvent>>,
    ) -> ExecResult<ExecOutput> {
        use russh_extra::russh::ChannelMsg;
        let session = self.session().await?;
        let guard = session
            .russh_handle()
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;
        let mut channel = guard
            .channel_open_session()
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;
        let command = remote_command(program, args, self.sudo_password.is_some());
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
            let message = if let Some(sender) = &stream {
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
                ChannelMsg::ExitStatus { exit_status } => exit = Some(exit_status),
                ChannelMsg::Close => break,
                _ => {}
            }
        }
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
                stderr: result.stderr.trim().into(),
            });
        }
        Ok(result)
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
