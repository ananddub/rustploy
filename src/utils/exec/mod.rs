mod error;
mod exec_local;
mod exec_remote;
mod types;

pub use error::{ExecError, ExecResult};
pub use exec_local::LocalExecutor;
pub use exec_remote::RemoteExecutor;
pub use types::{
    CommandExecutor, ExecExitStatus, ExecOutput, ExecStreamEvent, SshAuth, SshHostKey,
};
