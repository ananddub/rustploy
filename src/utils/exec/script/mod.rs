pub use cmd::IntoCommand;
pub use condition::Condition;
pub use pipeline::{ScriptPipeline, IfBuilder, IfThenBuilder};
pub use dsl::ShellIR;
pub use rustploy_sh_macros::sh;

pub fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

pub mod cmd;
pub mod condition;
pub mod macros;
pub mod pipeline;
pub mod dsl;
#[cfg(test)]
pub mod tests;

