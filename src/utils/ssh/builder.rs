use std::os::unix::fs::PermissionsExt;
use std::io::Write;
use std::path::PathBuf;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tempfile::NamedTempFile;
use tokio::process::Command;
use crate::utils::exec::{SshAuth, SshHostKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TtyMode {
    NoTty,     // -T
    NormalTty, // -t
    ForceTty,  // -tt
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrictHostKeyChecking {
    Yes,
    No,
    AcceptNew,
    Ask,
}

pub struct SshCommand {
    pub command: Command,
    pub temp_key_file: Option<NamedTempFile>,
}

pub struct SshBuilder {
    // Core parameters (Required)
    host: String,
    username: String,
    auth: SshAuth,
    host_key: SshHostKey,

    // Core options (Default-enabled, editable)
    multiplexing_enabled: bool,
    control_path: Option<PathBuf>,
    control_persist: String,

    // Optional parameters (Chainable)
    port: Option<u16>,
    tty: Option<TtyMode>,
    strict_host_key: Option<StrictHostKeyChecking>,
    known_hosts_file: Option<PathBuf>,
    known_hosts_command: Option<String>,
    connect_timeout: Option<u32>,
    server_alive_interval: Option<u32>,
    server_alive_count_max: Option<u32>,
    compression: Option<bool>,
    quiet: Option<bool>,
    verbosity: Option<u8>,
    config_file: Option<PathBuf>,
    ipv4_only: Option<bool>,
    ipv6_only: Option<bool>,
    local_forwards: Vec<String>,
    remote_forwards: Vec<String>,
    dynamic_forwards: Vec<String>,
    custom_options: Vec<(String, String)>,
}

fn quote(value: &str) -> String {
    if value.is_empty() {
        return "''".into();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

impl SshBuilder {
    pub fn new(host: String, username: String, auth: SshAuth, host_key: SshHostKey) -> Self {
        Self {
            host,
            username,
            auth,
            host_key,
            multiplexing_enabled: true,
            control_path: None,
            control_persist: "10m".to_string(),
            port: None,
            tty: None,
            strict_host_key: None,
            known_hosts_file: None,
            known_hosts_command: None,
            connect_timeout: Some(10),
            server_alive_interval: None,
            server_alive_count_max: None,
            compression: None,
            quiet: None,
            verbosity: None,
            config_file: None,
            ipv4_only: None,
            ipv6_only: None,
            local_forwards: Vec::new(),
            remote_forwards: Vec::new(),
            dynamic_forwards: Vec::new(),
            custom_options: Vec::new(),
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn disable_multiplexing(mut self) -> Self {
        self.multiplexing_enabled = false;
        self
    }

    pub fn control_multiplexing(mut self, path: PathBuf, persist_duration: impl Into<String>) -> Self {
        self.multiplexing_enabled = true;
        self.control_path = Some(path);
        self.control_persist = persist_duration.into();
        self
    }

    pub fn tty(mut self, mode: TtyMode) -> Self {
        self.tty = Some(mode);
        self
    }

    pub fn strict_host_key_checking(mut self, checking: StrictHostKeyChecking) -> Self {
        self.strict_host_key = Some(checking);
        self
    }

    pub fn user_known_hosts_file(mut self, path: PathBuf) -> Self {
        self.known_hosts_file = Some(path);
        self
    }

    pub fn known_hosts_command(mut self, cmd: impl Into<String>) -> Self {
        self.known_hosts_command = Some(cmd.into());
        self
    }

    pub fn connect_timeout(mut self, seconds: u32) -> Self {
        self.connect_timeout = Some(seconds);
        self
    }

    pub fn server_alive_interval(mut self, seconds: u32) -> Self {
        self.server_alive_interval = Some(seconds);
        self
    }

    pub fn server_alive_count_max(mut self, count: u32) -> Self {
        self.server_alive_count_max = Some(count);
        self
    }

    pub fn compression(mut self, enabled: bool) -> Self {
        self.compression = Some(enabled);
        self
    }

    pub fn quiet(mut self, enabled: bool) -> Self {
        self.quiet = Some(enabled);
        self
    }

    pub fn verbose(mut self, level: u8) -> Self {
        self.verbosity = Some(level);
        self
    }

    pub fn config_file(mut self, path: PathBuf) -> Self {
        self.config_file = Some(path);
        self
    }

    pub fn ipv4_only(mut self) -> Self {
        self.ipv4_only = Some(true);
        self.ipv6_only = None;
        self
    }

    pub fn ipv6_only(mut self) -> Self {
        self.ipv6_only = Some(true);
        self.ipv4_only = None;
        self
    }

    pub fn local_forward(mut self, forward_spec: impl Into<String>) -> Self {
        self.local_forwards.push(forward_spec.into());
        self
    }

    pub fn remote_forward(mut self, forward_spec: impl Into<String>) -> Self {
        self.remote_forwards.push(forward_spec.into());
        self
    }

    pub fn dynamic_forward(mut self, forward_spec: impl Into<String>) -> Self {
        self.dynamic_forwards.push(forward_spec.into());
        self
    }

    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_options.push((key.into(), value.into()));
        self
    }

    /// Internal option-push helper to prevent string-concatenation space issues
    fn push_option(args: &mut Vec<String>, key: &str, value: &str) {
        args.push("-o".to_string());
        args.push(format!("{}={}", key, value));
    }

    /// Prepares arguments and writes private key to secure temporary file (0600)
    pub fn build_args(&self) -> Result<(Vec<String>, Option<NamedTempFile>, Option<PathBuf>), std::io::Error> {
        // Validation: quiet and verbose are mutually exclusive in OpenSSH CLI
        if self.quiet == Some(true) && self.verbosity.is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Quiet (-q) and Verbose (-v) options are mutually exclusive.",
            ));
        }

        let mut args = Vec::new();
        let mut temp_key_file = None;
        let mut agent_socket_path = None;

        // Port
        if let Some(port) = self.port {
            args.push("-p".to_string());
            args.push(port.to_string());
        }

        // BatchMode: Always enabled for automation
        Self::push_option(&mut args, "BatchMode", "yes");

        // Strict Host Key strategy configuration
        let is_insecure = matches!(self.host_key, SshHostKey::InsecureAcceptAny);
        
        if let Some(cmd) = &self.known_hosts_command {
            Self::push_option(&mut args, "KnownHostsCommand", cmd);
        } else if let Some(checking) = &self.strict_host_key {
            let val = match checking {
                StrictHostKeyChecking::Yes => "yes",
                StrictHostKeyChecking::No => "no",
                StrictHostKeyChecking::AcceptNew => "accept-new",
                StrictHostKeyChecking::Ask => "ask",
            };
            Self::push_option(&mut args, "StrictHostKeyChecking", val);
        } else {
            // Apply secure derived default from SshHostKey enum
            match &self.host_key {
                SshHostKey::InsecureAcceptAny => {
                    Self::push_option(&mut args, "StrictHostKeyChecking", "no");
                    Self::push_option(&mut args, "UserKnownHostsFile", "/dev/null");
                }
                SshHostKey::PinnedSha256(fingerprint) => {
                    // Force StrictHostKeyChecking to yes to reject mismatched keys
                    Self::push_option(&mut args, "StrictHostKeyChecking", "yes");
                    
                    // Escape fingerprint to make it shell-safe for the script
                    let escaped_fingerprint = fingerprint.replace('\'', "'\\''");
                    let cmd = format!(
                        "sh -c 'if [ \"$2\" = \"$5\" ] || [ \"$2\" = \"SHA256:$5\" ]; then echo \"$1 $3 $4\"; fi' -- %H %F %t %K '{}'",
                        escaped_fingerprint
                    );
                    Self::push_option(&mut args, "KnownHostsCommand", &cmd);
                }
            }
        }

        // Only push UserKnownHostsFile if we are not in InsecureAcceptAny mode
        if !is_insecure {
            if let Some(path) = &self.known_hosts_file {
                Self::push_option(&mut args, "UserKnownHostsFile", &path.to_string_lossy());
            }
        } else if self.known_hosts_file.is_some() {
            tracing::warn!("UserKnownHostsFile is ignored because InsecureAcceptAny host key policy is active.");
        }

        // Connection Timeout
        if let Some(timeout) = self.connect_timeout {
            Self::push_option(&mut args, "ConnectTimeout", &timeout.to_string());
        }
        if let Some(interval) = self.server_alive_interval {
            Self::push_option(&mut args, "ServerAliveInterval", &interval.to_string());
        }
        if let Some(max_count) = self.server_alive_count_max {
            Self::push_option(&mut args, "ServerAliveCountMax", &max_count.to_string());
        }

        // Multiplexing
        if self.multiplexing_enabled {
            Self::push_option(&mut args, "ControlMaster", "auto");
            
            // Computes unique ControlPath based on destination socket configuration
            let resolved_path = match &self.control_path {
                Some(path) => path.clone(),
                None => {
                    let mut hasher = DefaultHasher::new();
                    self.host.hash(&mut hasher);
                    self.username.hash(&mut hasher);
                    self.port.unwrap_or(22).hash(&mut hasher);
                    let hash_val = hasher.finish();
                    PathBuf::from(format!("/tmp/rustploy-ssh-{:x}", hash_val))
                }
            };
            Self::push_option(&mut args, "ControlPath", &resolved_path.to_string_lossy());
            Self::push_option(&mut args, "ControlPersist", &self.control_persist);
        }

        // Compression
        if let Some(comp) = self.compression {
            Self::push_option(&mut args, "Compression", if comp { "yes" } else { "no" });
        }

        // Quiet mode
        if let Some(q) = self.quiet {
            if q {
                args.push("-q".to_string());
            }
        }

        // Verbosity
        if let Some(v) = self.verbosity {
            match v {
                1 => args.push("-v".to_string()),
                2 => args.push("-vv".to_string()),
                3 => args.push("-vvv".to_string()),
                _ => {}
            }
        }

        // Config file
        if let Some(path) = &self.config_file {
            args.push("-F".to_string());
            args.push(path.to_string_lossy().to_string());
        }

        // IP Versions
        if Some(true) == self.ipv4_only {
            args.push("-4".to_string());
        }
        if Some(true) == self.ipv6_only {
            args.push("-6".to_string());
        }

        // TTY allocation
        if let Some(tty_mode) = &self.tty {
            match tty_mode {
                TtyMode::NoTty => args.push("-T".to_string()),
                TtyMode::NormalTty => args.push("-t".to_string()),
                TtyMode::ForceTty => args.push("-tt".to_string()),
            }
        }

        // Port forwards
        for spec in &self.local_forwards {
            args.push("-L".to_string());
            args.push(spec.clone());
        }
        for spec in &self.remote_forwards {
            args.push("-R".to_string());
            args.push(spec.clone());
        }
        for spec in &self.dynamic_forwards {
            args.push("-D".to_string());
            args.push(spec.clone());
        }

        // Auth / Identity
        match &self.auth {
            SshAuth::KeyPair { private_key, .. } => {
                let mut temp_file = tempfile::Builder::new()
                    .prefix("rustploy-ssh-key-")
                    .tempfile()?;
                temp_file.write_all(private_key.as_bytes())?;
                
                let mut permissions = std::fs::metadata(temp_file.path())?.permissions();
                permissions.set_mode(0o600);
                std::fs::set_permissions(temp_file.path(), permissions)?;

                args.push("-i".to_string());
                args.push(temp_file.path().to_string_lossy().to_string());
                temp_key_file = Some(temp_file);
            }
            SshAuth::KeyFile(path) => {
                // Ensure key file has safe permissions (0600) on Unix platforms
                #[cfg(unix)]
                {
                    let metadata = std::fs::metadata(path)?;
                    let mode = metadata.permissions().mode();
                    if mode & 0o077 != 0 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!(
                                "Insecure private key file permissions: {:o}. Must be 0600 (owner read/write only).",
                                mode & 0o777
                            ),
                        ));
                    }
                }
                args.push("-i".to_string());
                args.push(path.to_string_lossy().to_string());
            }
            SshAuth::Agent => {
                // Hard block insecure ambient agent usage in production
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "Insecure ambient SSH Agent is disabled in production. Use AgentWithSocket(PathBuf) for multi-tenant isolation.",
                ));
            }
            SshAuth::AgentWithSocket(socket) => {
                Self::push_option(&mut args, "IdentitiesOnly", "no");
                agent_socket_path = Some(socket.clone());
            }
            SshAuth::Password(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "Password authentication is unsupported in native CLI mode (use keypairs)",
                ));
            }
        }

        // Custom options
        for (k, v) in &self.custom_options {
            Self::push_option(&mut args, k, v);
        }

        // User & Host
        args.push(format!("{}@{}", self.username, self.host));

        Ok((args, temp_key_file, agent_socket_path))
    }

    /// Builds the `SshCommand` holding command + temp file ownership
    pub fn build_command(&self, program: &str, program_args: &[String]) -> Result<SshCommand, std::io::Error> {
        let (mut args, temp_file, agent_socket) = self.build_args()?;
        
        let quoted_cmd = std::iter::once(program.to_string())
            .chain(program_args.iter().cloned())
            .map(|a| quote(&a))
            .collect::<Vec<_>>()
            .join(" ");

        args.push(quoted_cmd);

        let mut command = Command::new("ssh");
        command.args(args);

        if let Some(socket) = agent_socket {
            command.env("SSH_AUTH_SOCK", socket);
        }

        Ok(SshCommand {
            command,
            temp_key_file: temp_file,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_builder_defaults() {
        let builder = SshBuilder::new(
            "1.2.3.4".to_string(),
            "deploy".to_string(),
            SshAuth::KeyFile(PathBuf::from("/tmp/dummy_key")),
            SshHostKey::InsecureAcceptAny,
        );

        let (args, temp_key, agent_socket) = builder.build_args().unwrap();
        assert!(temp_key.is_none());
        assert!(agent_socket.is_none());

        // BatchMode=yes must be present
        assert!(args.contains(&"BatchMode=yes".to_string()));

        // InsecureAcceptAny must use UserKnownHostsFile=/dev/null and StrictHostKeyChecking=no
        assert!(args.contains(&"UserKnownHostsFile=/dev/null".to_string()));
        assert!(args.contains(&"StrictHostKeyChecking=no".to_string()));

        // Multiplexing must be auto enabled by default
        assert!(args.contains(&"ControlMaster=auto".to_string()));
        assert!(args.iter().any(|arg| arg.starts_with("ControlPath=")));
        assert!(args.contains(&"ControlPersist=10m".to_string()));
    }

    #[test]
    fn test_pinned_sha256_known_hosts_command() {
        let fingerprint = "SHA256:uNiVv6W1nE1G5fHqJqF5fK4zL7/zN5lK3y/8K6=";
        let builder = SshBuilder::new(
            "1.2.3.4".to_string(),
            "deploy".to_string(),
            SshAuth::KeyFile(PathBuf::from("/tmp/dummy_key")),
            SshHostKey::PinnedSha256(fingerprint.to_string()),
        );

        let (args, _, _) = builder.build_args().unwrap();
        
        // StrictHostKeyChecking=yes must be set to prevent fallback
        assert!(args.contains(&"StrictHostKeyChecking=yes".to_string()));

        // KnownHostsCommand must be configured
        assert!(args.iter().any(|arg| arg.starts_with("KnownHostsCommand=")));
        let kh_cmd = args.iter().find(|arg| arg.starts_with("KnownHostsCommand=")).unwrap();
        assert!(kh_cmd.contains("SHA256:uNiVv6W1nE1G5fHqJqF5fK4zL7/zN5lK3y/8K6="));
    }

    #[test]
    fn test_agent_with_socket_isolation() {
        let socket_path = PathBuf::from("/run/user/1000/ssh-agent.sock");
        let builder = SshBuilder::new(
            "1.2.3.4".to_string(),
            "deploy".to_string(),
            SshAuth::AgentWithSocket(socket_path.clone()),
            SshHostKey::InsecureAcceptAny,
        );

        let (args, _, agent_socket) = builder.build_args().unwrap();
        assert_eq!(agent_socket, Some(socket_path));
        
        // IdentitiesOnly=no must be set so that agent is queried
        assert!(args.contains(&"IdentitiesOnly=no".to_string()));
    }

    #[test]
    fn test_quiet_and_verbose_mutual_exclusivity() {
        let builder = SshBuilder::new(
            "1.2.3.4".to_string(),
            "deploy".to_string(),
            SshAuth::KeyFile(PathBuf::from("/tmp/dummy_key")),
            SshHostKey::InsecureAcceptAny,
        )
        .quiet(true)
        .verbose(2);

        let res = builder.build_args();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "Quiet (-q) and Verbose (-v) options are mutually exclusive."
        );
    }

    #[test]
    fn test_ip_version_flags() {
        let builder = SshBuilder::new(
            "1.2.3.4".to_string(),
            "deploy".to_string(),
            SshAuth::KeyFile(PathBuf::from("/tmp/dummy_key")),
            SshHostKey::InsecureAcceptAny,
        )
        .ipv4_only();

        let (args, _, _) = builder.build_args().unwrap();
        assert!(args.contains(&"-4".to_string()));
        assert!(!args.contains(&"-6".to_string()));
    }
}
