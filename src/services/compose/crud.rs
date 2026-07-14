use crate::api::dto::compose::{CreateComposeDto, PatchComposeDto};

use super::{
    ComposeRecord, ComposeService,
    queries::{generate_app_name, select_compose_by_id},
};

impl ComposeService {
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
        let compose_type = input.compose_type.unwrap_or(current.compose_type.as_str().to_string());
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

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM compose_projects WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}
