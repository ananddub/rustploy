use super::spec::{ComposeRuntime, ComposeSource, ComposeSpec};
use crate::{
    db::models::{domains::Domain, mounts::Mount},
    utils::builder::{
        env::generate_env_app,
        spec::{DomainSpec, MountKind, MountSpec},
    },
};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct ComposeSpecAdapter {
    db: Arc<SqlitePool>,
}

impl ComposeSpecAdapter {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn load(&self, compose_id: i64) -> sqlx::Result<ComposeSpec> {
        let compose = sqlx::query_as::<_, ComposeRow>(
            r#"SELECT
               c.app_name, c.source_type, c.compose_type, c.compose_file, c.env_var,
               c.repository, c.owner, c.branch, c.gitlab_repository, c.gitlab_owner,
               c.gitlab_branch, c.gitea_repository, c.gitea_branch, c.bitbucket_repository,
               c.bitbucket_owner, c.bitbucket_branch, c.custom_git_url, c.custom_git_branch,
               c.enable_submodules, c.compose_path, e.env_var AS environment_env,
               p.env_var AS project_env
               FROM compose_projects c
               JOIN environments e ON e.id = c.environment_id
               JOIN projects p ON p.id = e.project_id
               WHERE c.id = ?"#,
        )
        .bind(compose_id)
        .fetch_one(self.db.as_ref())
        .await?;
        let domains = sqlx::query_as::<_, Domain>(
            "SELECT id,host,https,port,path,internal_path,custom_entrypoint,service_name,custom_cert_resolver,strip_path,middlewares,domain_type,certificate_type,application_id,compose_id,created_at,updated_at FROM domains WHERE compose_id=? ORDER BY id",
        )
        .bind(compose_id)
        .fetch_all(self.db.as_ref())
        .await?;
        let mounts = sqlx::query_as::<_, Mount>(
            "SELECT id,mount_type,service_type,host_path,volume_name,file_path,content,mount_path,postgres_id,mysql_id,mariadb_id,mongo_id,redis_id,libsql_id,compose_id,application_id,created_at,updated_at FROM mounts WHERE compose_id=? ORDER BY id",
        )
        .bind(compose_id)
        .fetch_all(self.db.as_ref())
        .await?;
        convert(compose, domains, mounts).map_err(sqlx::Error::Protocol)
    }
}

#[derive(sqlx::FromRow)]
struct ComposeRow {
    app_name: String,
    source_type: String,
    compose_type: String,
    compose_file: String,
    env_var: Option<String>,
    repository: Option<String>,
    owner: Option<String>,
    branch: Option<String>,
    gitlab_repository: Option<String>,
    gitlab_owner: Option<String>,
    gitlab_branch: Option<String>,
    gitea_repository: Option<String>,
    gitea_branch: Option<String>,
    bitbucket_repository: Option<String>,
    bitbucket_owner: Option<String>,
    bitbucket_branch: Option<String>,
    custom_git_url: Option<String>,
    custom_git_branch: Option<String>,
    enable_submodules: i64,
    compose_path: String,
    environment_env: String,
    project_env: String,
}

fn convert(
    compose: ComposeRow,
    domains: Vec<Domain>,
    mounts: Vec<Mount>,
) -> Result<ComposeSpec, String> {
    let app_dir = format!("/etc/rustploy/compose/{}", compose.app_name);
    Ok(ComposeSpec {
        app_name: compose.app_name.clone(),
        stack_name: compose.app_name.clone(),
        source: source(&compose)?,
        runtime: runtime(&compose.compose_type)?,
        work_directory: format!("{app_dir}/source"),
        compose_path: if compose.source_type == "RAW" {
            format!("{app_dir}/compose.yml")
        } else {
            compose.compose_path
        },
        rendered_stack_file: format!("{app_dir}/rendered-stack.yml"),
        env_file: format!("{app_dir}/.env"),
        environment: generate_env_app(
            compose.environment_env,
            compose.project_env,
            compose.env_var.unwrap_or_default(),
        ),
        mounts: mounts
            .into_iter()
            .map(|mount| compose_mount(&compose.app_name, mount))
            .collect::<Result<Vec<_>, _>>()?,
        domains: domains
            .into_iter()
            .map(domain)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn source(compose: &ComposeRow) -> Result<ComposeSource, String> {
    let branch = |value: &Option<String>| value.clone().unwrap_or_else(|| "main".into());
    match compose.source_type.as_str() {
        "RAW" => Ok(ComposeSource::Raw {
            content: compose.compose_file.clone(),
        }),
        "GIT" => Ok(ComposeSource::Git {
            url: compose
                .custom_git_url
                .clone()
                .ok_or("custom_git_url is required")?,
            branch: branch(&compose.custom_git_branch),
            submodules: compose.enable_submodules != 0,
        }),
        "GITHUB" => Ok(ComposeSource::Git {
            url: format!(
                "https://github.com/{}/{}.git",
                compose.owner.as_deref().ok_or("owner is required")?,
                compose
                    .repository
                    .as_deref()
                    .ok_or("repository is required")?
            ),
            branch: branch(&compose.branch),
            submodules: compose.enable_submodules != 0,
        }),
        "GITLAB" => Ok(ComposeSource::Git {
            url: provider_url(
                "https://gitlab.com",
                compose.gitlab_owner.as_deref(),
                compose.gitlab_repository.as_deref(),
            )?,
            branch: branch(&compose.gitlab_branch),
            submodules: compose.enable_submodules != 0,
        }),
        "BITBUCKET" => Ok(ComposeSource::Git {
            url: provider_url(
                "https://bitbucket.org",
                compose.bitbucket_owner.as_deref(),
                compose.bitbucket_repository.as_deref(),
            )?,
            branch: branch(&compose.bitbucket_branch),
            submodules: compose.enable_submodules != 0,
        }),
        "GITEA" => {
            let url = compose
                .gitea_repository
                .clone()
                .filter(|value| value.contains("://") || value.starts_with("git@"))
                .ok_or("Gitea repository must be a full URL")?;
            Ok(ComposeSource::Git {
                url,
                branch: branch(&compose.gitea_branch),
                submodules: compose.enable_submodules != 0,
            })
        }
        other => Err(format!("unsupported compose source type: {other}")),
    }
}

fn runtime(value: &str) -> Result<ComposeRuntime, String> {
    match value {
        "DOCKER-COMPOSE" => Ok(ComposeRuntime::Compose),
        "STACK" => Ok(ComposeRuntime::Stack),
        other => Err(format!("unsupported compose type: {other}")),
    }
}

fn domain(domain: Domain) -> Result<DomainSpec, String> {
    Ok(DomainSpec {
        key: domain.id.unwrap_or_default().to_string(),
        host: domain.host,
        https: domain.https != 0,
        port: u16::try_from(domain.port.unwrap_or(3000)).map_err(|_| "invalid domain port")?,
        service_name: domain.service_name,
        path: domain.path.unwrap_or_else(|| "/".into()),
        internal_path: domain.internal_path.unwrap_or_else(|| "/".into()),
        strip_path: domain.strip_path != 0,
        entrypoint: domain.custom_entrypoint,
        certificate_type: domain.certificate_type,
        custom_cert_resolver: domain.custom_cert_resolver,
        middlewares: serde_json::from_str(&domain.middlewares).unwrap_or_default(),
    })
}

fn compose_mount(app: &str, mount: Mount) -> Result<MountSpec, String> {
    let is_file = mount.mount_type == "FILE";
    let kind = match mount.mount_type.as_str() {
        "VOLUME" => MountKind::Volume,
        "BIND" => MountKind::Bind,
        "FILE" => MountKind::File,
        other => return Err(format!("unsupported mount type: {other}")),
    };
    let source = match kind {
        MountKind::Volume => mount.volume_name.ok_or("volume_name is required")?,
        MountKind::Bind => mount.host_path.ok_or("host_path is required")?,
        MountKind::File => format!(
            "/etc/rustploy/compose/{app}/files/{}",
            mount.id.unwrap_or_default()
        ),
    };
    Ok(MountSpec {
        kind,
        source,
        target: mount.mount_path,
        read_only: is_file,
        content: mount.content,
    })
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
