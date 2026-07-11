use crate::utils::{
    builder::spec::ApplicationSpec,
    exec::{ExecError, ExecResult},
};

pub(super) fn validate_spec(spec: &ApplicationSpec) -> ExecResult<()> {
    for (label, value) in [
        ("application", spec.app_name.as_str()),
        ("stack", spec.stack_name.as_str()),
        ("network", spec.network.as_str()),
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
        if !domain.path.starts_with('/') || !domain.internal_path.starts_with('/') {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: "domain paths must start with /".into(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::builder::application::stack::tests::spec;

    #[test]
    fn unsafe_app_name_is_rejected() {
        let mut value = spec();
        value.app_name = "../../root".into();
        assert!(validate_spec(&value).is_err());
    }
}
