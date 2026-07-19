use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub mod builder;
pub mod command;
pub mod create;
pub mod delete;
pub mod exists;
pub mod temp;
pub mod walk;

pub use builder::DirBuilder;
pub use command::DirCommandBuilder;
pub use create::DirCreateBuilder;
pub use delete::DirDeleteBuilder;
pub use exists::DirExistsBuilder;
pub use temp::DirTempBuilder;
pub use walk::DirWalkBuilder;

pub struct DirCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> DirCli<'a> {
    pub fn current(&self) -> DirCommandBuilder<'a> {
        DirCommandBuilder::new(self.executor, "pwd", vec![])
    }
    pub fn change(&self, path: impl IntoCommand) -> DirCommandBuilder<'a> {
        DirCommandBuilder::new(self.executor, "cd", vec![path.build_str()])
    }
    pub fn temp(&self) -> DirTempBuilder<'a> {
        DirTempBuilder::new(self.executor)
    }
    pub fn dir(&self, path: impl IntoCommand) -> DirBuilder<'a> {
        DirBuilder::new(self.executor, path)
    }
}
