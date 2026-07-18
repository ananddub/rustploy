pub use error::{ExecError, ExecResult};
pub use exec_local::LocalExecutor;
pub use exec_remote::{RemoteExecutor, RemoteTerminal};
pub use types::{
    CommandExecutor, ExecExitStatus, ExecOutput, ExecStreamEvent, SshAuth, SshHostKey,
};
pub use arg_builder::ArgBuilder;
pub use pipeline::{ScriptPipeline, IntoCommand};

pub mod arg_builder;
pub mod error;
pub mod exec_local;
pub mod exec_remote;
pub mod types;
pub mod pipeline;
