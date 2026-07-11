use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
    time::Duration,
};

use auto_di::{resolve, singleton};
use sqlx::Row;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::api::dto::compose::{
    CreateComposeDto, PatchComposeBitbucketSourceDto, PatchComposeCustomGitSourceDto,
    PatchComposeDto, PatchComposeGiteaSourceDto, PatchComposeGithubSourceDto,
    PatchComposeGitlabSourceDto, PatchComposeRawSourceDto,
};
use crate::utils::{
    builder::{
        compose::{ComposeBuilder, adapter::ComposeSpecAdapter},
        custom_type::IdType,
        hash_state::ApplicationState,
        spec::BuilderEvent,
    },
    exec::{CommandExecutor, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey},
    session::RemoteExecutorRegistry,
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

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
        spawn_recover_stale_deployments(db.clone());
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

        let running_deployment = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status = 'RUNNING')",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?
            != 0;
        if running_deployment {
            return Err(sqlx::Error::Protocol(
                "compose deployment already running".into(),
            ));
        }

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
        let deployment = sqlx::query(
            r#"INSERT INTO deployments (title, description, status, state, log_path, compose_id, server_id, started_at, last_state_at)
               VALUES (?, ?, 'RUNNING', 'QUEUE', ?, ?, ?, strftime('%s', 'now'), strftime('%s', 'now'))
               RETURNING id"#,
        )
        .bind(operation.title())
        .bind(Some(format!("{} requested for {}", operation.as_str(), compose.name)))
        .bind(log_path)
        .bind(id)
        .bind(compose.server_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        let deployment_id: i64 = deployment.get("id");
        self.spawn_operation(id, deployment_id, operation);
        Ok(ComposeOperationResult {
            compose,
            deployment_id: Some(deployment_id),
            operation,
        })
    }

    pub async fn cancel_operation(&self, id: i64) -> sqlx::Result<bool> {
        self.get_by_id(id).await?;
        let has_running_deployment = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status = 'RUNNING')",
        )
        .bind(id)
        .fetch_one(self.db.as_ref())
        .await?
            != 0;
        if !has_running_deployment {
            return Ok(false);
        }

        let state = resolve::<ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        if !state.cancel_by_id(IdType::ComposeId(id)) {
            return Ok(false);
        }

        sqlx::query(
            "UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE compose_id = ? AND status = 'RUNNING'",
        )
        .bind(id)
        .execute(self.db.as_ref())
        .await?;
        Ok(true)
    }

    fn spawn_operation(&self, compose_id: i64, deployment_id: i64, operation: ComposeOperation) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let result = execute_operation(db.clone(), compose_id, deployment_id, operation).await;
            let (compose_status, deployment_status, error_message) = match result {
                Ok(()) => ("DONE", "DONE", None),
                Err(error) if is_cancelled_error(&error) => {
                    tracing::warn!(compose_id, deployment_id, operation = operation.as_str(), error = %error, "compose operation cancelled");
                    ("ERROR", "CANCELLED", Some(error))
                }
                Err(error) => {
                    tracing::error!(compose_id, deployment_id, operation = operation.as_str(), error = %error, "compose operation failed");
                    ("ERROR", "ERROR", Some(error))
                }
            };
            if let Err(error) =
                sqlx::query("UPDATE compose_projects SET compose_status = ? WHERE id = ?")
                    .bind(compose_status)
                    .bind(compose_id)
                    .execute(db.as_ref())
                    .await
            {
                tracing::error!(compose_id, error = %error, "could not persist compose status");
            }
            if let Err(error) = sqlx::query(
                "UPDATE deployments SET status = ?, state = ?, error_message = ?, finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE id = ?",
            )
            .bind(deployment_status)
            .bind(deployment_status)
            .bind(error_message)
            .bind(deployment_id)
            .execute(db.as_ref())
            .await
            {
                tracing::error!(deployment_id, error = %error, "could not persist compose deployment status");
            }
            if let Ok(state) = resolve::<ApplicationState>().await {
                state.remove_state(IdType::ComposeId(compose_id));
            }
        });
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM compose_projects WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}

async fn execute_operation(
    db: Arc<SqlitePool>,
    compose_id: i64,
    deployment_id: i64,
    operation: ComposeOperation,
) -> Result<(), String> {
    let spec = ComposeSpecAdapter::new(db.clone())
        .load(compose_id)
        .await
        .map_err(|error| format!("could not load compose deployment configuration: {error}"))?;
    let (environment_id, project_id, server_id) = sqlx::query_as::<_, (i64, i64, Option<i64>)>(
        r#"SELECT c.environment_id, e.project_id, c.server_id
           FROM compose_projects c JOIN environments e ON e.id = c.environment_id
           WHERE c.id = ?"#,
    )
    .bind(compose_id)
    .fetch_one(db.as_ref())
    .await
    .map_err(|error| format!("could not resolve compose context: {error}"))?;
    let compose_key = IdType::ComposeId(compose_id);
    let state = resolve::<ApplicationState>()
        .await
        .map_err(|error| format!("could not resolve application state: {error}"))?;
    state.reset_default(compose_key.clone(), environment_id, project_id);
    let cancel = state
        .cancellation_token(compose_key.clone())
        .unwrap_or_else(CancellationToken::new);
    let executor = match server_id {
        Some(server_id) => {
            let pid_file = deployment_pid_file(deployment_id);
            sqlx::query("UPDATE deployments SET pid = ? WHERE id = ?")
                .bind(&pid_file)
                .bind(deployment_id)
                .execute(db.as_ref())
                .await
                .map_err(|error| format!("could not persist remote compose pid file: {error}"))?;
            CommandExecutor::Remote(
                remote_executor(db.as_ref(), server_id)
                    .await?
                    .with_job_pid_file(pid_file),
            )
        }
        None => CommandExecutor::Local(LocalExecutor::new()),
    };
    let (events_tx, events_rx) = mpsc::channel(64);
    tokio::spawn(record_builder_events(db.clone(), deployment_id, events_rx));
    let builder = ComposeBuilder::new(executor)
        .with_state(state, compose_key)
        .with_events(events_tx);
    match operation {
        ComposeOperation::Stop => builder.stop(&spec).await.map_err(|error| error.to_string()),
        _ => builder
            .deploy(&spec, &cancel)
            .await
            .map(|_| ())
            .map_err(|error| error.to_string()),
    }
}

async fn record_builder_events(
    db: Arc<SqlitePool>,
    deployment_id: i64,
    mut events: mpsc::Receiver<BuilderEvent>,
) {
    while let Some(event) = events.recv().await {
        let state = builder_event_state(&event);
        let message = match &event {
            BuilderEvent::Failed(error) => Some(error.as_str()),
            _ => None,
        };
        if let Err(error) = sqlx::query(
            "UPDATE deployments SET state = ?, error_message = COALESCE(?, error_message), last_state_at = strftime('%s', 'now') WHERE id = ? AND status = 'RUNNING'",
        )
        .bind(state)
        .bind(message)
        .bind(deployment_id)
        .execute(db.as_ref())
        .await
        {
            tracing::error!(deployment_id, error = %error, "could not persist compose builder event");
        }
    }
}

fn builder_event_state(event: &BuilderEvent) -> &'static str {
    match event {
        BuilderEvent::Preparing => "PREPARING",
        BuilderEvent::SourceReady => "SOURCE_READY",
        BuilderEvent::Building => "BUILDING",
        BuilderEvent::ImageReady => "IMAGE_READY",
        BuilderEvent::Deploying => "DEPLOYING",
        BuilderEvent::Routing => "ROUTING",
        BuilderEvent::HealthCheck => "HEALTH_CHECK",
        BuilderEvent::Deployed => "DEPLOYED",
        BuilderEvent::Cancelled => "CANCELLED",
        BuilderEvent::Failed(_) => "FAILED",
    }
}

fn is_cancelled_error(error: &str) -> bool {
    error.to_ascii_lowercase().contains("cancel")
}

fn spawn_recover_stale_deployments(db: Arc<SqlitePool>) {
    tokio::spawn(async move {
        cleanup_stale_remote_jobs(db.clone(), "compose").await;
        if let Err(error) = sqlx::query(
            "UPDATE deployments SET status = 'ERROR', state = 'RECOVERED_AFTER_RESTART', error_message = COALESCE(error_message, 'server restarted while compose deployment was running'), finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE status = 'RUNNING' AND compose_id IS NOT NULL",
        )
        .execute(db.as_ref())
        .await
        {
            tracing::error!(error = %error, "could not recover stale compose deployments");
        }
        if let Err(error) = sqlx::query(
            "UPDATE compose_projects SET compose_status = 'ERROR' WHERE compose_status = 'RUNNING' AND id IN (SELECT compose_id FROM deployments WHERE state = 'RECOVERED_AFTER_RESTART' AND compose_id IS NOT NULL)",
        )
        .execute(db.as_ref())
        .await
        {
            tracing::error!(error = %error, "could not recover stale compose statuses");
        }
        spawn_recovered_remote_cleanup_retry(db.clone(), "compose");
    });
}

async fn cleanup_stale_remote_jobs(db: Arc<SqlitePool>, kind: &'static str) {
    let rows = match sqlx::query_as::<_, (i64, i64, String)>(
        "SELECT id, server_id, pid FROM deployments WHERE status = 'RUNNING' AND compose_id IS NOT NULL AND server_id IS NOT NULL AND pid IS NOT NULL",
    )
    .fetch_all(db.as_ref())
    .await
    {
        Ok(rows) => rows,
        Err(error) => {
            tracing::error!(error = %error, "could not load stale compose remote jobs");
            return;
        }
    };

    for (deployment_id, server_id, pid_file) in rows {
        match remote_executor(db.as_ref(), server_id).await {
            Ok(executor) => {
                if let Err(error) = executor.kill_pid_file(&pid_file).await {
                    tracing::warn!(
                        deployment_id,
                        server_id,
                        pid_file = %pid_file,
                        error = %error,
                        "failed to cleanup stale remote compose job after restart"
                    );
                } else {
                    clear_deployment_pid(db.clone(), deployment_id).await;
                    tracing::warn!(
                        deployment_id,
                        server_id,
                        pid_file = %pid_file,
                        kind,
                        "cleaned stale remote compose job after restart"
                    );
                }
            }
            Err(error) => {
                tracing::warn!(
                    deployment_id,
                    server_id,
                    pid_file = %pid_file,
                    error = %error,
                    "could not create remote executor for stale compose cleanup"
                );
            }
        }
    }
}

fn spawn_recovered_remote_cleanup_retry(db: Arc<SqlitePool>, kind: &'static str) {
    tokio::spawn(async move {
        for attempt in 1..=20 {
            let pending = cleanup_recovered_remote_jobs(db.clone(), kind, attempt).await;
            if pending == 0 {
                return;
            }
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
}

async fn cleanup_recovered_remote_jobs(
    db: Arc<SqlitePool>,
    kind: &'static str,
    attempt: usize,
) -> usize {
    let rows = match sqlx::query_as::<_, (i64, i64, String)>(
        "SELECT id, server_id, pid FROM deployments WHERE status = 'ERROR' AND state = 'RECOVERED_AFTER_RESTART' AND compose_id IS NOT NULL AND server_id IS NOT NULL AND pid IS NOT NULL",
    )
    .fetch_all(db.as_ref())
    .await
    {
        Ok(rows) => rows,
        Err(error) => {
            tracing::error!(error = %error, "could not load recovered compose remote jobs");
            return 0;
        }
    };

    let mut pending = 0;
    for (deployment_id, server_id, pid_file) in rows {
        match remote_executor(db.as_ref(), server_id).await {
            Ok(executor) => match executor.kill_pid_file(&pid_file).await {
                Ok(()) => {
                    clear_deployment_pid(db.clone(), deployment_id).await;
                    tracing::warn!(
                        deployment_id,
                        server_id,
                        pid_file = %pid_file,
                        kind,
                        attempt,
                        "cleaned recovered remote compose job"
                    );
                }
                Err(error) => {
                    pending += 1;
                    tracing::warn!(
                        deployment_id,
                        server_id,
                        pid_file = %pid_file,
                        error = %error,
                        kind,
                        attempt,
                        "remote compose cleanup retry failed"
                    );
                }
            },
            Err(error) => {
                pending += 1;
                tracing::warn!(
                    deployment_id,
                    server_id,
                    pid_file = %pid_file,
                    error = %error,
                    kind,
                    attempt,
                    "could not create remote executor for recovered compose cleanup retry"
                );
            }
        }
    }
    pending
}

async fn clear_deployment_pid(db: Arc<SqlitePool>, deployment_id: i64) {
    if let Err(error) = sqlx::query("UPDATE deployments SET pid = NULL WHERE id = ?")
        .bind(deployment_id)
        .execute(db.as_ref())
        .await
    {
        tracing::error!(deployment_id, error = %error, "could not clear compose deployment pid file");
    }
}

fn deployment_pid_file(deployment_id: i64) -> String {
    format!("/tmp/rustploy-deployment-{deployment_id}.pid")
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
        "compose deployment SSH host key verification is disabled because no fingerprint is stored for this server"
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
