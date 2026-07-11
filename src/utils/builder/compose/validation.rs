use crate::utils::{
    builder::compose::spec::ComposeSpec,
    exec::{ExecError, ExecResult},
};

pub(super) fn validate_spec(spec: &ComposeSpec) -> ExecResult<()> {
    for (label, value) in [
        ("compose", spec.app_name.as_str()),
        ("stack", spec.stack_name.as_str()),
    ] {
        if value.is_empty()
            || !value
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: format!("invalid {label} name: {value}"),
            });
        }
    }
    if !spec.compose_file_path().ends_with(".yml") && !spec.compose_file_path().ends_with(".yaml") {
        return Err(ExecError::CommandFailed {
            code: None,
            stderr: "compose file must end with .yml or .yaml".into(),
        });
    }
    for domain in &spec.domains {
        if domain.host.is_empty()
            || !domain
                .host
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '*'))
        {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: format!("invalid domain host: {}", domain.host),
            });
        }
    }
    Ok(())
}
