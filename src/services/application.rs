use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};

use auto_di::{resolve, singleton};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::api::dto::application::{
    CreateApplicationDto, PatchApplicationDto, PatchBitbucketSourceDto, PatchBuildConfigDto,
    PatchCustomGitSourceDto, PatchDockerSourceDto, PatchDropSourceDto, PatchGiteaSourceDto,
    PatchGithubSourceDto, PatchGitlabSourceDto, PatchResourceConfigDto,
};
use crate::utils::{
    builder::{
        adapter::ApplicationSpecAdapter, application::ApplicationBuilder, custom_type::IdType,
        hash_state::ApplicationState,
    },
    exec::{CommandExecutor, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey},
    session::RemoteExecutorRegistry,
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct ApplicationRecord {
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub source_type: String,
    pub build_type: String,
    pub app_status: String,
    pub trigger_type: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub build_server_id: Option<i64>,
    pub registry_id: Option<i64>,
    pub env_var: Option<String>,
    pub icon: Option<String>,
    pub repository: Option<String>,
    pub owner: Option<String>,
    pub branch: Option<String>,
    pub gitlab_repository: Option<String>,
    pub gitlab_owner: Option<String>,
    pub gitlab_branch: Option<String>,
    pub gitea_repository: Option<String>,
    pub gitea_owner: Option<String>,
    pub gitea_branch: Option<String>,
    pub bitbucket_repository: Option<String>,
    pub bitbucket_owner: Option<String>,
    pub bitbucket_branch: Option<String>,
    pub docker_image: Option<String>,
    pub registry_url: Option<String>,
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum ApplicationOperation {
    Deploy,
    Redeploy,
    Rebuild,
    Reload,
    Start,
}

impl ApplicationOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deploy => "deploy",
            Self::Redeploy => "redeploy",
            Self::Rebuild => "rebuild",
            Self::Reload => "reload",
            Self::Start => "start",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Deploy => "Application deploy",
            Self::Redeploy => "Application redeploy",
            Self::Rebuild => "Application rebuild",
            Self::Reload => "Application reload",
            Self::Start => "Application start",
        }
    }

    fn target_status(self) -> &'static str {
        match self {
            Self::Deploy | Self::Redeploy | Self::Rebuild | Self::Reload | Self::Start => "RUNNING",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationOperationResult {
    pub application: ApplicationRecord,
    pub deployment_id: Option<i64>,
    pub operation: ApplicationOperation,
}

pub struct ApplicationService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl ApplicationService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<ApplicationRecord> {
        select_application_by_id(self.db.as_ref(), id).await
    }

    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<ApplicationRecord>> {
        sqlx::query_as!(
            ApplicationRecord,
            r#"SELECT id AS "id!: i64", name, app_name, description, source_type, build_type, app_status, trigger_type,
               environment_id, server_id, build_server_id, registry_id, env_var, icon,
               repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
               gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
               bitbucket_branch, docker_image, registry_url, custom_git_url, custom_git_branch,
               created_at, updated_at
               FROM applications
               WHERE environment_id = ?
               ORDER BY created_at DESC, id DESC"#,
            environment_id
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn create(&self, input: CreateApplicationDto) -> sqlx::Result<ApplicationRecord> {
        let app_name = generate_app_name(&input.name);

        sqlx::query_as!(
            ApplicationRecord,
            r#"INSERT INTO applications
               (name, app_name, description, source_type, build_type, environment_id, server_id)
               VALUES (?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS "id!: i64", name, app_name, description, source_type, build_type, app_status, trigger_type,
               environment_id, server_id, build_server_id, registry_id, env_var, icon,
               repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
               gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
               bitbucket_branch, docker_image, registry_url, custom_git_url, custom_git_branch,
               created_at, updated_at"#,
            input.name,
            app_name,
            input.description,
            input.source_type,
            input.build_type,
            input.environment_id,
            input.server_id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn patch(
        &self,
        id: i64,
        input: PatchApplicationDto,
    ) -> sqlx::Result<ApplicationRecord> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let build_type = input.build_type.unwrap_or(current.build_type);
        let trigger_type = input.trigger_type.unwrap_or(current.trigger_type);
        let env_var = input.env_var.or(current.env_var);
        let icon = input.icon.or(current.icon);
        let server_id = input.server_id.or(current.server_id);
        let build_server_id = input.build_server_id.or(current.build_server_id);
        let registry_id = input.registry_id.or(current.registry_id);

        sqlx::query_as!(
            ApplicationRecord,
            r#"UPDATE applications
               SET name = ?, description = ?, build_type = ?, trigger_type = ?, env_var = ?,
                   icon = ?, server_id = ?, build_server_id = ?, registry_id = ?
               WHERE id = ?
               RETURNING id AS "id!: i64", name, app_name, description, source_type, build_type, app_status, trigger_type,
               environment_id, server_id, build_server_id, registry_id, env_var, icon,
               repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
               gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
               bitbucket_branch, docker_image, registry_url, custom_git_url, custom_git_branch,
               created_at, updated_at"#,
            name,
            description,
            build_type,
            trigger_type,
            env_var,
            icon,
            server_id,
            build_server_id,
            registry_id,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn set_github_source(
        &self,
        id: i64,
        input: PatchGithubSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'GITHUB',
               repository = ?, owner = ?, branch = ?, build_path = ?, github_provider_id = ?, auto_deploy = ?,
               gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
               gitlab_path_namespace = NULL, gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            input.repository,
            input.owner,
            input.branch,
            input.build_path.unwrap_or_else(|| "/".into()),
            input.github_provider_id,
            input.auto_deploy.unwrap_or(1),
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_gitlab_source(
        &self,
        id: i64,
        input: PatchGitlabSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'GITLAB',
               gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?,
               gitlab_build_path = ?, gitlab_path_namespace = ?, gitlab_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitea_repository = NULL, gitea_owner = NULL,
               gitea_branch = NULL, bitbucket_repository = NULL, bitbucket_repository_slug = NULL,
               bitbucket_owner = NULL, bitbucket_branch = NULL, docker_image = NULL, docker_username = NULL,
               docker_password = NULL, registry_url = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            input.gitlab_project_id,
            input.gitlab_repository,
            input.gitlab_owner,
            input.gitlab_branch,
            input.gitlab_build_path.unwrap_or_else(|| "/".into()),
            input.gitlab_path_namespace,
            input.gitlab_provider_id,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_gitea_source(
        &self,
        id: i64,
        input: PatchGiteaSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'GITEA',
               gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, gitea_build_path = ?, gitea_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            input.gitea_repository,
            input.gitea_owner,
            input.gitea_branch,
            input.gitea_build_path.unwrap_or_else(|| "/".into()),
            input.gitea_provider_id,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_bitbucket_source(
        &self,
        id: i64,
        input: PatchBitbucketSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'BITBUCKET',
               bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?,
               bitbucket_branch = ?, bitbucket_build_path = ?, bitbucket_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            input.bitbucket_repository,
            input.bitbucket_repository_slug,
            input.bitbucket_owner,
            input.bitbucket_branch,
            input.bitbucket_build_path.unwrap_or_else(|| "/".into()),
            input.bitbucket_provider_id,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_docker_source(
        &self,
        id: i64,
        input: PatchDockerSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'DOCKER',
               docker_image = ?, docker_username = ?, docker_password = ?, registry_url = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            input.docker_image,
            input.docker_username,
            input.docker_password,
            input.registry_url,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_custom_git_source(
        &self,
        id: i64,
        input: PatchCustomGitSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'GIT',
               custom_git_url = ?, custom_git_branch = ?, custom_git_build_path = ?, custom_git_ssh_key_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            input.custom_git_url,
            input.custom_git_branch,
            input.custom_git_build_path.unwrap_or_else(|| "/".into()),
            input.custom_git_ssh_key_id,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_drop_source(
        &self,
        id: i64,
        input: PatchDropSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'DROP',
               drop_build_path = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            input.drop_build_path.unwrap_or_else(|| "/".into()),
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn patch_build_config(
        &self,
        id: i64,
        input: PatchBuildConfigDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               build_args = COALESCE(?, build_args), build_secrets = COALESCE(?, build_secrets),
               dockerfile = COALESCE(?, dockerfile), docker_context_path = COALESCE(?, docker_context_path),
               docker_build_stage = COALESCE(?, docker_build_stage), publish_directory = COALESCE(?, publish_directory),
               is_static_spa = COALESCE(?, is_static_spa), create_env_file = COALESCE(?, create_env_file),
               railpack_version = COALESCE(?, railpack_version), heroku_version = COALESCE(?, heroku_version),
               command = COALESCE(?, command), args = COALESCE(?, args), build_path = COALESCE(?, build_path),
               clean_cache = COALESCE(?, clean_cache), enable_submodules = COALESCE(?, enable_submodules),
               watch_paths = COALESCE(?, watch_paths)
               WHERE id = ?"#,
            input.build_args,
            input.build_secrets,
            input.dockerfile,
            input.docker_context_path,
            input.docker_build_stage,
            input.publish_directory,
            input.is_static_spa,
            input.create_env_file,
            input.railpack_version,
            input.heroku_version,
            input.command,
            input.args,
            input.build_path,
            input.clean_cache,
            input.enable_submodules,
            input.watch_paths,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn patch_resource_config(
        &self,
        id: i64,
        input: PatchResourceConfigDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit),
               cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit),
               replicas = COALESCE(?, replicas)
               WHERE id = ?"#,
            input.memory_reservation,
            input.memory_limit,
            input.cpu_reservation,
            input.cpu_limit,
            input.replicas,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn run_operation(
        &self,
        id: i64,
        operation: ApplicationOperation,
    ) -> sqlx::Result<ApplicationOperationResult> {
        let mut tx = self.db.begin().await?;

        let app = sqlx::query_as!(
            ApplicationRecord,
            r#"UPDATE applications SET app_status = ? WHERE id = ?
               RETURNING id AS "id!: i64", name, app_name, description, source_type, build_type, app_status, trigger_type,
               environment_id, server_id, build_server_id, registry_id, env_var, icon,
               repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
               gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
               bitbucket_branch, docker_image, registry_url, custom_git_url, custom_git_branch,
               created_at, updated_at"#,
            operation.target_status(),
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        let log_path = format!("logs/applications/{}/{}.log", id, Uuid::new_v4());
        let deployment = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, log_path, application_id, server_id, started_at)
               VALUES (?, ?, 'RUNNING', ?, ?, ?, strftime('%s', 'now'))
               RETURNING id AS "id!: i64""#,
            operation.title(),
            Some(format!("{} requested for {}", operation.as_str(), app.name)),
            log_path,
            id,
            app.server_id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        self.spawn_operation(id, deployment.id, operation);
        Ok(ApplicationOperationResult {
            application: app,
            deployment_id: Some(deployment.id),
            operation,
        })
    }

    fn spawn_operation(
        &self,
        application_id: i64,
        deployment_id: i64,
        operation: ApplicationOperation,
    ) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let result = execute_operation(db.clone(), application_id, operation).await;
            let (application_status, deployment_status, error_message) = match result {
                Ok(()) => ("DONE", "DONE", None),
                Err(error) => {
                    tracing::error!(application_id, deployment_id, operation = operation.as_str(), error = %error, "application operation failed");
                    ("ERROR", "ERROR", Some(error))
                }
            };
            if let Err(error) = sqlx::query("UPDATE applications SET app_status = ? WHERE id = ?")
                .bind(application_status)
                .bind(application_id)
                .execute(db.as_ref())
                .await
            {
                tracing::error!(application_id, error = %error, "could not persist application status");
            }
            if let Err(error) = sqlx::query(
                "UPDATE deployments SET status = ?, error_message = ?, finished_at = strftime('%s', 'now') WHERE id = ?",
            )
            .bind(deployment_status)
            .bind(error_message)
            .bind(deployment_id)
            .execute(db.as_ref())
            .await
            {
                tracing::error!(deployment_id, error = %error, "could not persist deployment status");
            }
        });
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM applications WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}

async fn execute_operation(
    db: Arc<SqlitePool>,
    application_id: i64,
    _operation: ApplicationOperation,
) -> Result<(), String> {
    let spec = ApplicationSpecAdapter::new(db.clone())
        .load(application_id)
        .await
        .map_err(|error| format!("could not load deployment configuration: {error}"))?;
    let (environment_id, project_id, server_id) = sqlx::query_as::<_, (i64, i64, Option<i64>)>(
        r#"SELECT a.environment_id, e.project_id, a.server_id
           FROM applications a JOIN environments e ON e.id = a.environment_id
           WHERE a.id = ?"#,
    )
    .bind(application_id)
    .fetch_one(db.as_ref())
    .await
    .map_err(|error| format!("could not resolve deployment context: {error}"))?;

    let app_key = IdType::AppId(application_id);
    let state = resolve::<ApplicationState>()
        .await
        .map_err(|error| format!("could not resolve application state: {error}"))?;
    state.ensure_default(app_key.clone(), environment_id, project_id);
    let cancel = state
        .cancellation_token(app_key.clone())
        .unwrap_or_else(CancellationToken::new);

    let executor = match server_id {
        Some(server_id) => CommandExecutor::Remote(remote_executor(db.as_ref(), server_id).await?),
        None => CommandExecutor::Local(LocalExecutor::new()),
    };
    ApplicationBuilder::new(executor)
        .with_state(state, app_key)
        .deploy(&spec, &cancel)
        .await
        .map(|_| ())
        .map_err(|error| error.to_string())
}

async fn remote_executor(db: &SqlitePool, server_id: i64) -> Result<RemoteExecutor, String> {
    let row = sqlx::query_as::<_, (String, i64, String, String, String)>(
        r#"SELECT s.ip_address, s.port, s.username, k.private_key, k.public_key
           FROM servers s JOIN ssh_keys k ON k.id = s.ssh_key_id WHERE s.id = ?"#,
    )
    .bind(server_id)
    .fetch_one(db)
    .await
    .map_err(|error| format!("could not load SSH credentials: {error}"))?;
    let mut hasher = DefaultHasher::new();
    row.hash(&mut hasher);
    let version = hasher.finish();
    if let Some(executor) = RemoteExecutorRegistry::global().get(server_id, version) {
        return Ok(executor);
    }
    let port = u16::try_from(row.1).map_err(|_| "SSH port must be between 0 and 65535")?;
    tracing::warn!(
        server_id,
        "deployment SSH host key verification is disabled because no fingerprint is stored for this server"
    );
    let executor = RemoteExecutor::new(
        row.0,
        port,
        row.2,
        SshAuth::key_pair(row.3, row.4),
        SshHostKey::InsecureAcceptAny,
    )
    .with_pool_size(4)
    .with_sudo();
    RemoteExecutorRegistry::global().insert(server_id, version, executor.clone());
    Ok(executor)
}

async fn select_application_by_id(db: &SqlitePool, id: i64) -> sqlx::Result<ApplicationRecord> {
    sqlx::query_as!(
        ApplicationRecord,
        r#"SELECT id AS "id!: i64", name, app_name, description, source_type, build_type, app_status, trigger_type,
           environment_id, server_id, build_server_id, registry_id, env_var, icon,
           repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
           gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
           bitbucket_branch, docker_image, registry_url, custom_git_url, custom_git_branch,
           created_at, updated_at
           FROM applications WHERE id = ?"#,
        id
    )
    .fetch_one(db)
    .await
}

fn generate_app_name(name: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;

    for ch in name.to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            previous_dash = false;
        } else if !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }

    let slug = slug.trim_matches('-');
    let base = if slug.is_empty() { "app" } else { slug };
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}", base, &suffix[..6])
}
