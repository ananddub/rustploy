use super::spec::*;
use crate::db::models::{domains::Domain, mounts::Mount};
use sqlx::SqlitePool;
use std::{collections::BTreeMap, sync::Arc};
use crate::utils::builder::env::generate_env_app;

#[derive(Clone)]
pub struct ApplicationSpecAdapter {
    db: Arc<SqlitePool>,
}
impl ApplicationSpecAdapter {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }
    pub async fn load(&self, application_id: i64) -> sqlx::Result<ApplicationSpec> {
        let app=sqlx::query_as::<_,AppRow>(r#"SELECT a.id,a.app_name,a.source_type,a.build_type,a.build_args,a.build_secrets,a.dockerfile,a.docker_context_path,a.docker_build_stage,a.publish_directory,a.is_static_spa,a.command,a.args,a.env_var,a.build_path,a.clean_cache,a.memory_reservation,a.memory_limit,a.cpu_reservation,a.cpu_limit,a.replicas,a.health_check_swarm,a.placement_swarm,a.stop_grace_period_swarm,a.repository,a.owner,a.branch,a.gitlab_repository,a.gitlab_owner,a.gitlab_branch,a.gitea_repository,a.gitea_owner,a.gitea_branch,a.bitbucket_repository,a.bitbucket_owner,a.bitbucket_branch,a.docker_image,a.docker_username,a.docker_password,a.registry_url,a.custom_git_url,a.custom_git_branch,a.registry_id,e.env_var AS environment_env,p.env_var AS project_env,r.image_prefix AS registry_image_prefix,r.username AS registry_username,r.password AS registry_password,r.registry_url AS joined_registry_url FROM applications a JOIN environments e ON e.id=a.environment_id JOIN projects p ON p.id=e.project_id LEFT JOIN registries r ON r.id=a.registry_id WHERE a.id=?"#).bind(application_id).fetch_one(self.db.as_ref()).await?;
        let domains=sqlx::query_as::<_,Domain>("SELECT id,host,https,port,path,internal_path,custom_entrypoint,service_name,custom_cert_resolver,strip_path,middlewares,domain_type,certificate_type,application_id,compose_id,created_at,updated_at FROM domains WHERE application_id=? ORDER BY id").bind(application_id).fetch_all(self.db.as_ref()).await?;
        let mounts=sqlx::query_as::<_,Mount>("SELECT id,mount_type,service_type,host_path,volume_name,file_path,content,mount_path,postgres_id,mysql_id,mariadb_id,mongo_id,redis_id,libsql_id,compose_id,application_id,created_at,updated_at FROM mounts WHERE application_id=? ORDER BY id").bind(application_id).fetch_all(self.db.as_ref()).await?;
        self.convert(app, domains, mounts)
            .map_err(sqlx::Error::Protocol)
    }
    fn convert(
        &self,
        app: AppRow,
        domains: Vec<Domain>,
        mounts: Vec<Mount>,
    ) -> Result<ApplicationSpec, String> {
        let source = source(&app)?;
        let build = build(&app)?;
        let mut environment = generate_env_app(app.environment_env.clone(), app.project_env.clone(), app.env_var.clone().as_deref().unwrap_or("").into());
        // environment.extend(parse_env(&app.project_env));
        // environment.extend(parse_env(app.env_var.as_deref().unwrap_or("")));
        let registry = registry_auth(&app);
        let image = if app.source_type == "DOCKER" {
            app.docker_image.clone().ok_or("Docker image is required")?
        } else if let Some(prefix) = &app.registry_image_prefix {
            format!("{}/{}:latest", prefix.trim_end_matches('/'), app.app_name)
        } else {
            format!("{}:latest", app.app_name)
        };
        let source = match source {
            SourceSpec::Docker { image, .. } => SourceSpec::Docker { image, registry },
            other => other,
        };
        let work_directory = format!("/etc/rustploy/applications/{}/code", app.app_name);
        let domain_specs = domains
            .into_iter()
            .map(domain)
            .collect::<Result<Vec<_>, _>>()?;
        let mount_specs = mounts
            .into_iter()
            .map(|m| mount(&app.app_name, m))
            .collect::<Result<Vec<_>, _>>()?;
        let healthcheck = app
            .health_check_swarm
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .map(serde_json::from_str)
            .transpose()
            .map_err(|e| format!("invalid health_check_swarm: {e}"))?;
        let placement_constraints = app
            .placement_swarm
            .as_deref()
            .and_then(|v| serde_json::from_str::<serde_json::Value>(v).ok())
            .and_then(|v| {
                v.get("constraints")
                    .or_else(|| v.get("Constraints"))
                    .cloned()
            })
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();
        Ok(ApplicationSpec {
            app_name: app.app_name.clone(),
            stack_name: app.app_name.clone(),
            source,
            build,
            work_directory,
            image,
            environment,
            build_args: parse_env(app.build_args.as_deref().unwrap_or("")),
            build_secrets: parse_env(app.build_secrets.as_deref().unwrap_or("")),
            command: app.command.map(|v| shell_words(&v)),
            args: app
                .args
                .as_deref()
                .and_then(|v| serde_json::from_str(v).ok())
                .unwrap_or_default(),
            replicas: u32::try_from(app.replicas.max(1)).unwrap_or(1),
            network: "rustploy-network".into(),
            mounts: mount_specs,
            domains: domain_specs,
            resources: ResourceSpec {
                memory_limit: app.memory_limit,
                memory_reservation: app.memory_reservation,
                cpu_limit: app.cpu_limit,
                cpu_reservation: app.cpu_reservation,
            },
            healthcheck,
            placement_constraints,
            stop_grace_period: app.stop_grace_period_swarm.map(|v| format!("{v}s")),
        })
    }
}

#[derive(sqlx::FromRow)]
struct AppRow {
    id: i64,
    app_name: String,
    source_type: String,
    build_type: String,
    build_args: Option<String>,
    build_secrets: Option<String>,
    dockerfile: Option<String>,
    docker_context_path: Option<String>,
    docker_build_stage: Option<String>,
    publish_directory: Option<String>,
    is_static_spa: Option<i64>,
    command: Option<String>,
    args: Option<String>,
    env_var: Option<String>,
    build_path: Option<String>,
    clean_cache: i64,
    memory_reservation: Option<String>,
    memory_limit: Option<String>,
    cpu_reservation: Option<String>,
    cpu_limit: Option<String>,
    replicas: i64,
    health_check_swarm: Option<String>,
    placement_swarm: Option<String>,
    stop_grace_period_swarm: Option<i64>,
    repository: Option<String>,
    owner: Option<String>,
    branch: Option<String>,
    gitlab_repository: Option<String>,
    gitlab_owner: Option<String>,
    gitlab_branch: Option<String>,
    gitea_repository: Option<String>,
    gitea_owner: Option<String>,
    gitea_branch: Option<String>,
    bitbucket_repository: Option<String>,
    bitbucket_owner: Option<String>,
    bitbucket_branch: Option<String>,
    docker_image: Option<String>,
    docker_username: Option<String>,
    docker_password: Option<String>,
    registry_url: Option<String>,
    custom_git_url: Option<String>,
    custom_git_branch: Option<String>,
    registry_id: Option<i64>,
    environment_env: String,
    project_env: String,
    registry_image_prefix: Option<String>,
    registry_username: Option<String>,
    registry_password: Option<String>,
    joined_registry_url: Option<String>,
}
fn source(a: &AppRow) -> Result<SourceSpec, String> {
    let branch = |v: &Option<String>| v.clone().unwrap_or_else(|| "main".into());
    match a.source_type.as_str() {
        "DOCKER" => Ok(SourceSpec::Docker {
            image: a.docker_image.clone().ok_or("docker_image is required")?,
            registry: None,
        }),
        "GIT" => Ok(SourceSpec::Git {
            url: a
                .custom_git_url
                .clone()
                .ok_or("custom_git_url is required")?,
            branch: branch(&a.custom_git_branch),
            submodules: false,
        }),
        "GITHUB" => Ok(SourceSpec::Git {
            url: format!(
                "https://github.com/{}/{}.git",
                a.owner.as_deref().ok_or("owner is required")?,
                a.repository.as_deref().ok_or("repository is required")?
            ),
            branch: branch(&a.branch),
            submodules: false,
        }),
        "GITLAB" => Ok(SourceSpec::Git {
            url: provider_url(
                "https://gitlab.com",
                a.gitlab_owner.as_deref(),
                a.gitlab_repository.as_deref(),
            )?,
            branch: branch(&a.gitlab_branch),
            submodules: false,
        }),
        "BITBUCKET" => Ok(SourceSpec::Git {
            url: provider_url(
                "https://bitbucket.org",
                a.bitbucket_owner.as_deref(),
                a.bitbucket_repository.as_deref(),
            )?,
            branch: branch(&a.bitbucket_branch),
            submodules: false,
        }),
        "GITEA" => {
            let url = a
                .gitea_repository
                .clone()
                .filter(|v| v.contains("://") || v.starts_with("git@"))
                .ok_or("Gitea repository must be a full URL")?;
            Ok(SourceSpec::Git {
                url,
                branch: branch(&a.gitea_branch),
                submodules: false,
            })
        }
        other => Err(format!("unsupported source type: {other}")),
    }
}
fn build(a: &AppRow) -> Result<Option<BuildStrategy>, String> {
    if a.source_type == "DOCKER" {
        return Ok(None);
    }
    Ok(Some(match a.build_type.as_str() {
        "DOCKERFILE" => BuildStrategy::Dockerfile {
            dockerfile: a.dockerfile.clone().unwrap_or_else(|| "Dockerfile".into()),
            context: a.docker_context_path.clone().unwrap_or_else(|| ".".into()),
            target: a.docker_build_stage.clone(),
            no_cache: a.clean_cache != 0,
        },
        "NIXPACKS" => BuildStrategy::Nixpacks,
        "PAKETO_BUILDPACKS" => BuildStrategy::Paketo,
        "RAILPACK" => BuildStrategy::Railpack {
            version: "0.15.4".into(),
        },
        "STATIC" => BuildStrategy::Static {
            publish_directory: a
                .publish_directory
                .clone()
                .ok_or("publish_directory is required")?,
            spa: a.is_static_spa.unwrap_or(0) != 0,
        },
        other => return Err(format!("unsupported build type: {other}")),
    }))
}
fn registry_auth(a: &AppRow) -> Option<RegistryAuth> {
    let username = a
        .docker_username
        .clone()
        .or_else(|| a.registry_username.clone())?;
    let password = a
        .docker_password
        .clone()
        .or_else(|| a.registry_password.clone())?;
    Some(RegistryAuth {
        registry: a
            .registry_url
            .clone()
            .or_else(|| a.joined_registry_url.clone())
            .unwrap_or_default(),
        username,
        password,
    })
}
fn domain(d: Domain) -> Result<DomainSpec, String> {
    Ok(DomainSpec {
        key: d.id.unwrap_or_default().to_string(),
        host: d.host,
        https: d.https != 0,
        port: u16::try_from(d.port.unwrap_or(3000)).map_err(|_| "invalid domain port")?,
        path: d.path.unwrap_or_else(|| "/".into()),
        internal_path: d.internal_path.unwrap_or_else(|| "/".into()),
        strip_path: d.strip_path != 0,
        entrypoint: d.custom_entrypoint,
        certificate_type: d.certificate_type,
        custom_cert_resolver: d.custom_cert_resolver,
        middlewares: serde_json::from_str(&d.middlewares).unwrap_or_default(),
    })
}
fn mount(app: &str, m: Mount) -> Result<MountSpec, String> {
    let is_file = m.mount_type == "FILE";
    let kind = match m.mount_type.as_str() {
        "VOLUME" => MountKind::Volume,
        "BIND" => MountKind::Bind,
        "FILE" => MountKind::File,
        other => return Err(format!("unsupported mount type: {other}")),
    };
    let source = match kind {
        MountKind::Volume => m.volume_name.ok_or("volume_name is required")?,
        MountKind::Bind => m.host_path.ok_or("host_path is required")?,
        MountKind::File => format!(
            "/etc/rustploy/applications/{app}/files/{}",
            m.id.unwrap_or_default()
        ),
    };
    Ok(MountSpec {
        kind,
        source,
        target: m.mount_path,
        read_only: is_file,
        content: m.content,
    })
}
fn parse_env(value: &str) -> BTreeMap<String, String> {
    value
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (k, v) = line.split_once('=')?;
            Some((k.trim().into(), v.trim().into()))
        })
        .collect()
}
fn provider_url(base: &str, owner: Option<&str>, repo: Option<&str>) -> Result<String, String> {
    let repo = repo.ok_or("repository is required")?;
    if repo.contains("://") || repo.starts_with("git@") {
        Ok(repo.into())
    } else {
        Ok(format!(
            "{base}/{}/{}.git",
            owner.ok_or("owner is required")?,
            repo
        ))
    }
}
fn shell_words(value: &str) -> Vec<String> {
    value.split_whitespace().map(str::to_owned).collect()
}
