use crate::utils::builder::spec::{ApplicationSpec, DomainSpec};
use serde_json::{Map, Value, json};

pub fn application_config(app: &ApplicationSpec) -> Value {
    let mut routers = Map::new();
    let mut services = Map::new();
    let mut middlewares = Map::new();
    for domain in &app.domains {
        let names = domain_names(app, domain);
        let mut middleware_names = domain.middlewares.clone();
        if domain.internal_path != "/" && domain.internal_path != domain.path {
            let name = format!("addprefix-{}-{}", app.app_name, domain.key);
            middlewares.insert(
                name.clone(),
                json!({"addPrefix":{"prefix":domain.internal_path}}),
            );
            middleware_names.push(name);
        }
        if domain.strip_path && domain.path != "/" {
            let name = format!("stripprefix-{}-{}", app.app_name, domain.key);
            middlewares.insert(
                name.clone(),
                json!({"stripPrefix":{"prefixes":[domain.path]}}),
            );
            middleware_names.push(name);
        }
        let mut router = json!({"rule":rule(domain),"service":names.1,"entryPoints":[domain.entrypoint.clone().unwrap_or_else(||if domain.https{"websecure".into()}else{"web".into()})],"middlewares":middleware_names});
        if domain.https {
            let resolver = domain.custom_cert_resolver.clone().or_else(|| {
                (domain.certificate_type.eq_ignore_ascii_case("LETSENCRYPT"))
                    .then(|| "letsencrypt".into())
            });
            router["tls"] = resolver
                .map(|r| json!({"certResolver":r}))
                .unwrap_or_else(|| json!({}));
        }
        routers.insert(names.0, router);
        let service_target = domain
            .service_name
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| app.app_name.as_str());
        services.insert(names.1,json!({"loadBalancer":{"servers":[{"url":format!("http://{}:{}",service_target,domain.port)}],"passHostHeader":true}}));
    }
    json!({"http":{"routers":routers,"services":services,"middlewares":middlewares}})
}
fn domain_names(app: &ApplicationSpec, domain: &DomainSpec) -> (String, String) {
    (
        format!("{}-{}-router", app.app_name, domain.key),
        format!("{}-{}-service", app.app_name, domain.key),
    )
}
fn rule(domain: &DomainSpec) -> String {
    let host = format!("Host(`{}`)", domain.host);
    if domain.path != "/" {
        format!("{host} && PathPrefix(`{}`)", domain.path)
    } else {
        host
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::builder::spec::*;
    use std::collections::BTreeMap;
    #[test]
    fn generates_https_path_router_and_middlewares() {
        let app = ApplicationSpec {
            app_name: "api".into(),
            stack_name: "prod".into(),
            source: SourceSpec::Docker {
                image: "api:1".into(),
                registry: None,
            },
            build: None,
            work_directory: "/tmp".into(),
            image: "api:1".into(),
            environment: BTreeMap::new(),
            build_args: BTreeMap::new(),
            build_secrets: BTreeMap::new(),
            command: None,
            args: vec![],
            replicas: 1,
            network: "rustploy-network".into(),
            mounts: vec![],
            domains: vec![DomainSpec {
                key: "1".into(),
                host: "api.example.com".into(),
                https: true,
                port: 3000,
                service_name: None,
                path: "/v1".into(),
                internal_path: "/api".into(),
                strip_path: true,
                entrypoint: None,
                certificate_type: "LETSENCRYPT".into(),
                custom_cert_resolver: None,
                middlewares: vec![],
            }],
            resources: ResourceSpec::default(),
            healthcheck: None,
            placement_constraints: vec![],
            stop_grace_period: None,
        };
        let value = application_config(&app);
        assert_eq!(
            value["http"]["routers"]["api-1-router"]["tls"]["certResolver"],
            "letsencrypt"
        );
        assert!(
            value["http"]["middlewares"]
                .get("addprefix-api-1")
                .is_some()
        );
        assert!(
            value["http"]["middlewares"]
                .get("stripprefix-api-1")
                .is_some()
        );
    }
}
