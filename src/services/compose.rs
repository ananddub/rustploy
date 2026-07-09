use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::api::dto::compose::{
    CreateComposeDto, PatchComposeBitbucketSourceDto, PatchComposeCustomGitSourceDto,
    PatchComposeDto, PatchComposeGiteaSourceDto, PatchComposeGithubSourceDto,
    PatchComposeGitlabSourceDto, PatchComposeRawSourceDto,
};

#[derive(Debug, Clone)]
pub struct ComposeRecord {
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub env_var: Option<String>,
    pub compose_file: String,
    pub source_type: String,
    pub compose_type: String,
    pub compose_status: String,
    pub trigger_type: String,
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
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub command: String,
    pub compose_path: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum ComposeOperation {
    Deploy,
    Redeploy,
    Reload,
    Start,
    Stop,
}

impl ComposeOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deploy => "deploy",
            Self::Redeploy => "redeploy",
            Self::Reload => "reload",
            Self::Start => "start",
            Self::Stop => "stop",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Deploy => "Compose deploy",
            Self::Redeploy => "Compose redeploy",
            Self::Reload => "Compose reload",
            Self::Start => "Compose start",
            Self::Stop => "Compose stop",
        }
    }

    fn target_status(self) -> &'static str {
        match self {
            Self::Stop => "IDLE",
            _ => "RUNNING",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComposeOperationResult {
    pub compose: ComposeRecord,
    pub deployment_id: Option<i64>,
    pub operation: ComposeOperation,
}

pub struct ComposeService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl ComposeService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<ComposeRecord> {
        select_compose_by_id(self.db.as_ref(), id).await
    }

    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<ComposeRecord>> {
        sqlx::query_as!(
            ComposeRecord,
            r#"SELECT id AS "id!: i64", name, app_name, description, env_var, compose_file,
               source_type, compose_type, compose_status, trigger_type,
               repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
               gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
               bitbucket_branch, custom_git_url, custom_git_branch, command, compose_path,
               environment_id, server_id, created_at, updated_at
               FROM compose_projects
               WHERE environment_id = ?
               ORDER BY created_at DESC, id DESC"#,
            environment_id
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn create(&self, input: CreateComposeDto) -> sqlx::Result<ComposeRecord> {
        let app_name = generate_app_name(&input.name);

        sqlx::query_as!(
            ComposeRecord,
            r#"INSERT INTO compose_projects
               (name, app_name, description, environment_id, server_id, source_type, compose_type, compose_file)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS "id!: i64", name, app_name, description, env_var, compose_file,
               source_type, compose_type, compose_status, trigger_type,
               repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
               gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
               bitbucket_branch, custom_git_url, custom_git_branch, command, compose_path,
               environment_id, server_id, created_at, updated_at"#,
            input.name,
            app_name,
            input.description,
            input.environment_id,
            input.server_id,
            input.source_type,
            input.compose_type,
            input.compose_file
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn patch(&self, id: i64, input: PatchComposeDto) -> sqlx::Result<ComposeRecord> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let env_var = input.env_var.or(current.env_var);
        let compose_file = input.compose_file.unwrap_or(current.compose_file);
        let compose_type = input.compose_type.unwrap_or(current.compose_type);
        let trigger_type = input.trigger_type.unwrap_or(current.trigger_type);
        let command = input.command.unwrap_or(current.command);
        let compose_path = input.compose_path.unwrap_or(current.compose_path);
        let server_id = input.server_id.or(current.server_id);

        sqlx::query!(
            r#"UPDATE compose_projects SET
               name = ?, description = ?, env_var = ?, compose_file = ?, compose_type = ?,
               trigger_type = ?, command = ?, enable_submodules = COALESCE(?, enable_submodules),
               compose_path = ?, suffix = COALESCE(?, suffix), randomize = COALESCE(?, randomize),
               isolated_deployment = COALESCE(?, isolated_deployment),
               isolated_deployments_volume = COALESCE(?, isolated_deployments_volume),
               watch_paths = COALESCE(?, watch_paths), server_id = ?
               WHERE id = ?"#,
            name,
            description,
            env_var,
            compose_file,
            compose_type,
            trigger_type,
            command,
            input.enable_submodules,
            compose_path,
            input.suffix,
            input.randomize,
            input.isolated_deployment,
            input.isolated_deployments_volume,
            input.watch_paths,
            server_id,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_github_source(
        &self,
        id: i64,
        input: PatchComposeGithubSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'GITHUB',
               repository = ?, owner = ?, branch = ?, github_provider_id = ?, auto_deploy = ?,
               gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
               gitlab_path_namespace = NULL, gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            input.repository,
            input.owner,
            input.branch,
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
        input: PatchComposeGitlabSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'GITLAB',
               gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?,
               gitlab_path_namespace = ?, gitlab_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitea_repository = NULL, gitea_owner = NULL,
               gitea_branch = NULL, bitbucket_repository = NULL, bitbucket_repository_slug = NULL,
               bitbucket_owner = NULL, bitbucket_branch = NULL, custom_git_url = NULL,
               custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            input.gitlab_project_id,
            input.gitlab_repository,
            input.gitlab_owner,
            input.gitlab_branch,
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
        input: PatchComposeGiteaSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'GITEA',
               gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, gitea_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            input.gitea_repository,
            input.gitea_owner,
            input.gitea_branch,
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
        input: PatchComposeBitbucketSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'BITBUCKET',
               bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?,
               bitbucket_branch = ?, bitbucket_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            input.bitbucket_repository,
            input.bitbucket_repository_slug,
            input.bitbucket_owner,
            input.bitbucket_branch,
            input.bitbucket_provider_id,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_custom_git_source(
        &self,
        id: i64,
        input: PatchComposeCustomGitSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'GIT',
               custom_git_url = ?, custom_git_branch = ?, custom_git_ssh_key_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL
               WHERE id = ?"#,
            input.custom_git_url,
            input.custom_git_branch,
            input.custom_git_ssh_key_id,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_raw_source(
        &self,
        id: i64,
        input: PatchComposeRawSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'RAW', compose_file = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            input.compose_file,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn run_operation(
        &self,
        id: i64,
        operation: ComposeOperation,
    ) -> sqlx::Result<ComposeOperationResult> {
        let mut tx = self.db.begin().await?;

        let compose = sqlx::query_as!(
            ComposeRecord,
            r#"UPDATE compose_projects SET compose_status = ? WHERE id = ?
               RETURNING id AS "id!: i64", name, app_name, description, env_var, compose_file,
               source_type, compose_type, compose_status, trigger_type,
               repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
               gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
               bitbucket_branch, custom_git_url, custom_git_branch, command, compose_path,
               environment_id, server_id, created_at, updated_at"#,
            operation.target_status(),
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        let log_path = format!("logs/compose/{}/{}.log", id, Uuid::new_v4());
        let deployment = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, log_path, compose_id, server_id, started_at)
               VALUES (?, ?, 'RUNNING', ?, ?, ?, strftime('%s', 'now'))
               RETURNING id AS "id!: i64""#,
            operation.title(),
            Some(format!("{} requested for {}", operation.as_str(), compose.name)),
            log_path,
            id,
            compose.server_id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(ComposeOperationResult {
            compose,
            deployment_id: Some(deployment.id),
            operation,
        })
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM compose_projects WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}

async fn select_compose_by_id(db: &SqlitePool, id: i64) -> sqlx::Result<ComposeRecord> {
    sqlx::query_as!(
        ComposeRecord,
        r#"SELECT id AS "id!: i64", name, app_name, description, env_var, compose_file,
           source_type, compose_type, compose_status, trigger_type,
           repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
           gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
           bitbucket_branch, custom_git_url, custom_git_branch, command, compose_path,
           environment_id, server_id, created_at, updated_at
           FROM compose_projects WHERE id = ?"#,
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
    let base = if slug.is_empty() { "compose" } else { slug };
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}", base, &suffix[..6])
}
