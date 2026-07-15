use crate::db::models::{domains::Domain, mounts::Mount};
use crate::utils::builder::compose::spec::ComposeSpec;
use sqlx::SqlitePool;
use std::sync::Arc;

use super::mapper::{ComposeRow, ComposeRowWithRelations};

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
               c.custom_git_ssh_key_id, c.enable_submodules, c.compose_path, e.env_var AS environment_env,
               p.env_var AS project_env,
               ghp.github_private_key AS github_token,
               glp.access_token AS gitlab_token,
               bbp.api_token AS bitbucket_token,
               gtp.access_token AS gitea_token,
               sk.private_key AS ssh_private_key
               FROM compose_projects c
               JOIN environments e ON e.id = c.environment_id
               JOIN projects p ON p.id = e.project_id
               LEFT JOIN github_providers ghp ON ghp.id = c.github_provider_id
               LEFT JOIN gitlab_providers glp ON glp.id = c.gitlab_provider_id
               LEFT JOIN bitbucket_providers bbp ON bbp.id = c.bitbucket_provider_id
               LEFT JOIN gitea_providers gtp ON gtp.id = c.gitea_provider_id
               LEFT JOIN ssh_keys sk ON sk.id = c.custom_git_ssh_key_id
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
        let data = ComposeRowWithRelations { compose, domains, mounts };
        ComposeSpec::try_from(data).map_err(|e| sqlx::Error::Protocol(e.to_string()))
    }
}
