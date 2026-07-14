use super::{
    compose::ComposeBuilder,
    spec::{ComposeRuntime, ComposeSpec},
};
use crate::utils::{
    builder::spec::DomainSpec,
    exec::{ExecError, ExecResult},
};
use serde_yaml::{Mapping, Value};
use tokio_util::sync::CancellationToken;

const TRAEFIK_NETWORK: &str = "rustploy-network";

pub(super) async fn write_labeled_compose(
    builder: &ComposeBuilder,
    spec: &ComposeSpec,
    cancel: &CancellationToken,
) -> ExecResult<()> {
    if spec.domains.is_empty() {
        return Ok(());
    }

    let compose_path = spec.compose_file_path();
    let output = builder
        .executor
        .run_cancelled("cat", [compose_path.as_str()], cancel)
        .await?;
    let mut document = serde_yaml::from_str::<Value>(&output.stdout)
        .map_err(|error| command_error(format!("invalid compose yaml: {error}")))?;

    inject_domain_labels(&mut document, spec)?;

    let content = serde_yaml::to_string(&document)
        .map_err(|error| ExecError::Json(serde_json::Error::io(std::io::Error::other(error))))?;
    builder
        .write_file_cancelled(&compose_path, content.as_bytes(), cancel)
        .await
}

fn inject_domain_labels(document: &mut Value, spec: &ComposeSpec) -> ExecResult<()> {
    let root = mapping_mut(document, "compose root must be a yaml object")?;
    let services = root
        .get_mut(Value::String("services".into()))
        .ok_or_else(|| command_error("compose file has no services"))?;
    let services = mapping_mut(services, "compose services must be a yaml object")?;

    for domain in &spec.domains {
        let service_name = domain.service_name.as_deref().ok_or_else(|| {
            command_error(format!("domain {} is missing service_name", domain.host))
        })?;
        let service = services
            .get_mut(Value::String(service_name.into()))
            .ok_or_else(|| {
                command_error(format!(
                    "domain {} points to missing compose service {}",
                    domain.host, service_name
                ))
            })?;
        inject_labels_for_service(service, spec, domain)?;
        add_network_to_service(service)?;
    }

    add_network_to_root(root);
    Ok(())
}

fn inject_labels_for_service(
    service: &mut Value,
    spec: &ComposeSpec,
    domain: &DomainSpec,
) -> ExecResult<()> {
    let service = mapping_mut(service, "compose service must be a yaml object")?;
    let labels_value = match spec.runtime {
        ComposeRuntime::Compose => service
            .entry(Value::String("labels".into()))
            .or_insert_with(|| Value::Sequence(vec![])),
        ComposeRuntime::Stack => {
            let deploy = service
                .entry(Value::String("deploy".into()))
                .or_insert_with(|| Value::Mapping(Mapping::new()));
            mapping_mut(deploy, "compose deploy must be a yaml object")?
                .entry(Value::String("labels".into()))
                .or_insert_with(|| Value::Sequence(vec![]))
        }
    };

    insert_label(labels_value, "traefik.enable=true")?;
    match spec.runtime {
        ComposeRuntime::Compose => insert_label(
            labels_value,
            format!("traefik.docker.network={TRAEFIK_NETWORK}"),
        )?,
        ComposeRuntime::Stack => insert_label(
            labels_value,
            format!("traefik.docker.network={TRAEFIK_NETWORK}"),
        )?,
    }

    for label in create_domain_labels(spec, domain, domain.entrypoint.as_deref().unwrap_or("web")) {
        insert_label(labels_value, label)?;
    }
    if domain.entrypoint.is_none() && domain.https {
        for label in create_domain_labels(spec, domain, "websecure") {
            insert_label(labels_value, label)?;
        }
    }
    Ok(())
}

fn create_domain_labels(spec: &ComposeSpec, domain: &DomainSpec, entrypoint: &str) -> Vec<String> {
    let router_name = format!("{}-{}-{entrypoint}", spec.app_name, domain.key);
    let mut labels = vec![
        format!("traefik.http.routers.{router_name}.rule={}", rule(domain)),
        format!("traefik.http.routers.{router_name}.entrypoints={entrypoint}"),
        format!(
            "traefik.http.services.{router_name}.loadbalancer.server.port={}",
            domain.port
        ),
        format!("traefik.http.routers.{router_name}.service={router_name}"),
    ];

    let mut middlewares = Vec::new();
    let is_redirect_router = entrypoint == "web" && domain.https && domain.entrypoint.is_none();
    if is_redirect_router {
        middlewares.push("redirect-to-https@file".to_string());
    }
    if domain.strip_path && domain.path != "/" {
        let middleware = format!("stripprefix-{}-{}", spec.app_name, domain.key);
        if entrypoint == "web" || domain.entrypoint.is_some() {
            labels.push(format!(
                "traefik.http.middlewares.{middleware}.stripprefix.prefixes={}",
                domain.path
            ));
        }
        if !is_redirect_router {
            middlewares.push(middleware);
        }
    }
    if domain.internal_path != "/" && domain.internal_path.starts_with('/') {
        let middleware = format!("addprefix-{}-{}", spec.app_name, domain.key);
        if entrypoint == "web" || domain.entrypoint.is_some() {
            labels.push(format!(
                "traefik.http.middlewares.{middleware}.addprefix.prefix={}",
                domain.internal_path
            ));
        }
        if !is_redirect_router {
            middlewares.push(middleware);
        }
    }
    if !is_redirect_router {
        middlewares.extend(domain.middlewares.clone());
    }
    if !middlewares.is_empty() {
        labels.push(format!(
            "traefik.http.routers.{router_name}.middlewares={}",
            middlewares.join(",")
        ));
    }

    if entrypoint == "websecure" || (domain.entrypoint.is_some() && domain.https) {
        if domain.certificate_type.eq_ignore_ascii_case("LETSENCRYPT") {
            labels.push(format!(
                "traefik.http.routers.{router_name}.tls.certresolver=letsencrypt"
            ));
        } else if domain.certificate_type.eq_ignore_ascii_case("CUSTOM") {
            if let Some(resolver) = &domain.custom_cert_resolver {
                labels.push(format!(
                    "traefik.http.routers.{router_name}.tls.certresolver={resolver}"
                ));
            }
        } else if domain.https {
            labels.push(format!("traefik.http.routers.{router_name}.tls=true"));
        }
    }

    labels
}

fn add_network_to_service(service: &mut Value) -> ExecResult<()> {
    let service = mapping_mut(service, "compose service must be a yaml object")?;
    let networks = service
        .entry(Value::String("networks".into()))
        .or_insert_with(|| Value::Sequence(vec![]));
    match networks {
        Value::Sequence(items) => {
            push_unique(items, Value::String(TRAEFIK_NETWORK.into()));
            Ok(())
        }
        Value::Mapping(map) => {
            map.entry(Value::String(TRAEFIK_NETWORK.into()))
                .or_insert(Value::Null);
            Ok(())
        }
        _ => Err(command_error("service networks must be a list or object")),
    }
}

fn add_network_to_root(root: &mut Mapping) {
    let networks = root
        .entry(Value::String("networks".into()))
        .or_insert_with(|| Value::Mapping(Mapping::new()));
    if let Ok(map) = mapping_mut(networks, "root networks must be a yaml object") {
        let mut network = Mapping::new();
        network.insert(Value::String("external".into()), Value::Bool(true));
        network.insert(
            Value::String("name".into()),
            Value::String(TRAEFIK_NETWORK.into()),
        );
        map.entry(Value::String(TRAEFIK_NETWORK.into()))
            .or_insert(Value::Mapping(network));
    }
}

fn insert_label(labels: &mut Value, label: impl Into<String>) -> ExecResult<()> {
    let label = label.into();
    match labels {
        Value::Sequence(items) => {
            push_unique(items, Value::String(label));
            Ok(())
        }
        Value::Mapping(map) => {
            let Some((key, value)) = label.split_once('=') else {
                return Err(command_error("invalid traefik label"));
            };
            map.insert(Value::String(key.into()), Value::String(value.into()));
            Ok(())
        }
        _ => Err(command_error("labels must be a list or object")),
    }
}

fn push_unique(items: &mut Vec<Value>, value: Value) {
    if !items.iter().any(|item| item == &value) {
        items.insert(0, value);
    }
}

fn mapping_mut<'a>(value: &'a mut Value, message: &str) -> ExecResult<&'a mut Mapping> {
    match value {
        Value::Mapping(map) => Ok(map),
        _ => Err(command_error(message)),
    }
}

/// Build traefik labels grouped by Swarm service name.
/// Used for zero-redeploy label updates via `docker service update`.
/// Returns HashMap<service_name, Vec<label>>
pub fn build_compose_service_labels(
    app_name: &str,
    domains: &[crate::services::domain::DomainRecord],
) -> std::collections::HashMap<String, Vec<String>> {
    use std::collections::HashMap;
    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    for domain in domains {
        let service_name = match &domain.service_name {
            Some(s) if !s.is_empty() => {
                if s.starts_with(&format!("{app_name}_")) {
                    s.clone()
                } else {
                    format!("{app_name}_{s}")
                }
            }
            _ => continue, // skip domains without service_name for compose stack
        };

        let entry = result.entry(service_name.clone()).or_default();

        if entry.is_empty() {
            entry.push("traefik.enable=true".into());
            entry.push(format!("traefik.docker.network={TRAEFIK_NETWORK}"));
        }

        let key = domain.id.to_string();
        let entrypoint = domain.custom_entrypoint.clone()
            .unwrap_or_else(|| if domain.https != 0 { "websecure".into() } else { "web".into() });
        let router = format!("{app_name}-{key}-{entrypoint}");
        let path = domain.path.as_deref().unwrap_or("/");
        let host = &domain.host;
        let rule = if path != "/" {
            format!("Host(`{host}`) && PathPrefix(`{path}`)")
        } else {
            format!("Host(`{host}`)")
        };
        let port = domain.port.unwrap_or(3000);

        entry.push(format!("traefik.http.routers.{router}.rule={rule}"));
        entry.push(format!("traefik.http.routers.{router}.entrypoints={entrypoint}"));
        entry.push(format!("traefik.http.services.{router}.loadbalancer.server.port={port}"));
        entry.push(format!("traefik.http.routers.{router}.service={router}"));

        if domain.https != 0 {
            let cert = &domain.certificate_type;
            if cert.eq_ignore_ascii_case("LETSENCRYPT") {
                entry.push(format!("traefik.http.routers.{router}.tls.certresolver=letsencrypt"));
            } else {
                entry.push(format!("traefik.http.routers.{router}.tls=true"));
            }
        }
    }

    result
}

fn rule(domain: &DomainSpec) -> String {
    let host = format!("Host(`{}`)", domain.host);
    if domain.path != "/" {
        format!("{host} && PathPrefix(`{}`)", domain.path)
    } else {
        host
    }
}

fn command_error(message: impl Into<String>) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::builder::{
        compose::spec::{ComposeSource, ComposeSpec},
        spec::DomainSpec,
    };
    use std::collections::BTreeMap;

    #[test]
    fn injects_stack_labels_on_service_deploy() {
        let mut doc: Value = serde_yaml::from_str(
            r#"
services:
  web:
    image: nginx
"#,
        )
        .unwrap();
        inject_domain_labels(&mut doc, &spec(ComposeRuntime::Stack)).unwrap();
        let out = serde_yaml::to_string(&doc).unwrap();
        assert!(out.contains("traefik.enable=true"));
        assert!(out.contains("traefik.docker.network=rustploy-network"));
        assert!(out.contains("traefik.http.routers.site-1-websecure.tls.certresolver=letsencrypt"));
        assert!(out.contains("rustploy-network"));
    }

    #[test]
    fn injects_compose_labels_on_service_labels() {
        let mut doc: Value = serde_yaml::from_str(
            r#"
services:
  web:
    image: nginx
"#,
        )
        .unwrap();
        inject_domain_labels(&mut doc, &spec(ComposeRuntime::Compose)).unwrap();
        let out = serde_yaml::to_string(&doc).unwrap();
        assert!(out.contains("traefik.docker.network=rustploy-network"));
        assert!(out.contains("traefik.http.services.site-1-web.loadbalancer.server.port=3000"));
    }

    fn spec(runtime: ComposeRuntime) -> ComposeSpec {
        ComposeSpec {
            app_name: "site".into(),
            stack_name: "site".into(),
            source: ComposeSource::Raw {
                content: String::new(),
            },
            runtime,
            work_directory: "/tmp/site".into(),
            compose_path: "/tmp/site/compose.yml".into(),
            rendered_stack_file: "/tmp/site/rendered.yml".into(),
            env_file: "/tmp/site/.env".into(),
            environment: BTreeMap::new(),
            mounts: vec![],
            domains: vec![DomainSpec {
                key: "1".into(),
                host: "site.example.com".into(),
                https: true,
                port: 3000,
                service_name: Some("web".into()),
                path: "/".into(),
                internal_path: "/".into(),
                strip_path: false,
                entrypoint: None,
                certificate_type: "LETSENCRYPT".into(),
                custom_cert_resolver: None,
                middlewares: vec![],
            }],
        }
    }
}
