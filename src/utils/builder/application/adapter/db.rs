use crate::db::models::{domains::Domain, mounts::Mount};
use crate::utils::builder::spec::ApplicationSpec;
use sqlx::SqlitePool;
use std::sync::Arc;
use auto_di::singleton;

use super::mapper::{AppRow, AppRowWithRelations};

#[derive(Clone)]
pub struct ApplicationSpecAdapter {
    db: Arc<SqlitePool>,
}

#[singleton]
impl ApplicationSpecAdapter {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn load(&self, application_id: i64) -> sqlx::Result<ApplicationSpec> {
        let app = sqlx::query_as::<_, AppRow>(
            r#"SELECT
               a.app_name, a.source_type, a.build_type, a.build_args, a.build_secrets,
               a.dockerfile, a.docker_context_path, a.docker_build_stage,
               a.publish_directory, a.is_static_spa, a.build_path, a.command, a.args, a.env_var,
               a.clean_cache, a.memory_reservation, a.memory_limit,
               a.cpu_reservation, a.cpu_limit, a.replicas, a.health_check_swarm,
               a.placement_swarm, a.stop_grace_period_swarm, a.repository, a.owner,
               a.branch, a.gitlab_repository, a.gitlab_owner, a.gitlab_branch,
               a.gitea_repository, a.gitea_branch, a.bitbucket_repository,
               a.bitbucket_owner, a.bitbucket_branch, a.docker_image, a.docker_username,
               a.docker_password, a.registry_url, a.custom_git_url, a.custom_git_branch,
               a.custom_git_ssh_key_id,
               e.env_var AS environment_env, p.env_var AS project_env,
               r.image_prefix AS registry_image_prefix, r.username AS registry_username,
               r.password AS registry_password, r.registry_url AS joined_registry_url,
               ghp.github_private_key AS github_token,
               glp.access_token AS gitlab_token,
               bbp.api_token AS bitbucket_token,
               gtp.access_token AS gitea_token,
               sk.private_key AS ssh_private_key
               FROM applications a
               JOIN environments e ON e.id = a.environment_id
               JOIN projects p ON p.id = e.project_id
               LEFT JOIN registries r ON r.id = a.registry_id
               LEFT JOIN github_providers ghp ON ghp.id = a.github_provider_id
               LEFT JOIN gitlab_providers glp ON glp.id = a.gitlab_provider_id
               LEFT JOIN bitbucket_providers bbp ON bbp.id = a.bitbucket_provider_id
               LEFT JOIN gitea_providers gtp ON gtp.id = a.gitea_provider_id
               LEFT JOIN ssh_keys sk ON sk.id = a.custom_git_ssh_key_id
               WHERE a.id = ?"#,
        )
        .bind(application_id)
        .fetch_one(self.db.as_ref())
        .await?;
        let domains = sqlx::query_as::<_, Domain>(
            "SELECT id,host,https,port,path,internal_path,custom_entrypoint,service_name,custom_cert_resolver,strip_path,middlewares,domain_type,certificate_type,application_id,compose_id,created_at,updated_at FROM domains WHERE application_id=? ORDER BY id",
        )
        .bind(application_id)
        .fetch_all(self.db.as_ref())
        .await?;
        let mounts = sqlx::query_as::<_, Mount>(
            "SELECT id,mount_type,service_type,host_path,volume_name,file_path,content,mount_path,postgres_id,mysql_id,mariadb_id,mongo_id,redis_id,libsql_id,compose_id,application_id,created_at,updated_at FROM mounts WHERE application_id=? ORDER BY id",
        )
        .bind(application_id)
        .fetch_all(self.db.as_ref())
        .await?;
        let data = AppRowWithRelations { app, domains, mounts };
        ApplicationSpec::try_from(data).map_err(|e| sqlx::Error::Protocol(e.to_string()))
    }
}
