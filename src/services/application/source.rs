use crate::api::dto::application::{
    PatchBitbucketSourceDto, PatchCustomGitSourceDto, PatchDockerSourceDto, PatchDropSourceDto,
    PatchGiteaSourceDto, PatchGithubSourceDto, PatchGitlabSourceDto,
};

use super::{ApplicationRecord, ApplicationService};

impl ApplicationService {
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
}
