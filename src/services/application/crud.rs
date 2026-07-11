use crate::api::dto::application::{CreateApplicationDto, PatchApplicationDto};

use super::{
    ApplicationRecord, ApplicationService,
    queries::{generate_app_name, select_application_by_id},
};

impl ApplicationService {
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

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM applications WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}
