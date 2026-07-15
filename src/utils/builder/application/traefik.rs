use crate::utils::{
    builder::spec::ApplicationSpec,
    traefik::{
        middleware::Middleware, rule::Rule, traefik::TraefikBuilder, types::CertificateType,
    },
};

pub fn build_traefik_labels(app: &ApplicationSpec) -> Vec<String> {
    if app.domains.is_empty() {
        return vec![];
    }

    let mut traefik = TraefikBuilder::new().enable().network(&app.network);

    for domain in &app.domains {
        let router_name = format!("{}-{}", app.app_name, domain.key);

        let entrypoint = domain.entrypoint.clone().unwrap_or_else(|| {
            if domain.https {
                "websecure".into()
            } else {
                "web".into()
            }
        });

        // Build the routing rule from structured types.
        let rule = {
            let base = Rule::host(&domain.host);
            if domain.path != "/" {
                base.and(Rule::path_prefix(&domain.path))
            } else {
                base
            }
        };

        // --- Middlewares ---
        let mut middleware_names: Vec<String> = Vec::new();

        if domain.strip_path && domain.path != "/" {
            let name = format!("stripprefix-{router_name}");
            traefik = traefik.middleware(Middleware::StripPrefix {
                name: name.clone(),
                prefixes: vec![domain.path.clone()],
            });
            middleware_names.push(name);
        }

        if domain.internal_path != "/" && domain.internal_path != domain.path {
            let name = format!("addprefix-{router_name}");
            traefik = traefik.middleware(Middleware::AddPrefix {
                name: name.clone(),
                prefix: domain.internal_path.clone(),
            });
            middleware_names.push(name);
        }

        middleware_names.extend(domain.middlewares.clone());

        // --- Main router: build fully before finalizing with .service() ---
        let mut r = traefik
            .router(&router_name)
            .rule(&rule)
            .entrypoint(&entrypoint);

        if !middleware_names.is_empty() {
            r = r.middlewares(&middleware_names);
        }

        if domain.https {
            r = r.tls(true);
            let cert_type = CertificateType::from(domain.certificate_type.as_str());
            match cert_type {
                CertificateType::LetsEncrypt => {
                    r = r.cert_resolver("letsencrypt");
                }
                CertificateType::Custom => {
                    if let Some(resolver) = &domain.custom_cert_resolver {
                        r = r.cert_resolver(resolver);
                    }
                }
                CertificateType::None => {}
            }
        }

        // .service(name) finalizes the router and returns TraefikBuilder,
        // then we chain .service(name) (ServiceBuilder) -> .port() -> .finish().
        traefik = r
            .service(&router_name) // RouterBuilder → TraefikBuilder
            .service(&router_name) // TraefikBuilder → ServiceBuilder
            .port(domain.port) // ServiceBuilder → ServiceBuilder
            .finish(); // ServiceBuilder → TraefikBuilder

        // --- HTTP → HTTPS redirect router (only when no custom entrypoint is set) ---
        if domain.https && domain.entrypoint.is_none() {
            let redirect_name = format!("{router_name}-redirect");
            traefik = traefik
                .router(&redirect_name)
                .rule(&rule)
                .entrypoint("web")
                .middleware("redirect-to-https@file")
                .finish();
        }
    }

    traefik.build()
}
