use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub mod builder;
pub mod chmod;
pub mod chown;
pub mod delete;
pub mod exists;
pub mod read;
pub mod write;
pub mod replace;

pub use builder::FileBuilder;
pub use chmod::FileChmodBuilder;
pub use chown::FileChownBuilder;
pub use delete::FileDeleteBuilder;
pub use exists::FileExistsBuilder;
pub use read::FileReadBuilder;
pub use write::FileWriteBuilder;
pub use replace::FileReplaceBuilder;

pub struct FileCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> FileCli<'a> {
    pub fn file(&self, path: impl IntoCommand) -> FileBuilder<'a> {
        FileBuilder::new(self.executor, path)
    }
}
