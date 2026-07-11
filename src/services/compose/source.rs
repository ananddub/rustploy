use crate::api::dto::compose::{
    PatchComposeBitbucketSourceDto, PatchComposeCustomGitSourceDto, PatchComposeGiteaSourceDto,
    PatchComposeGithubSourceDto, PatchComposeGitlabSourceDto, PatchComposeRawSourceDto,
};

use super::{ComposeRecord, ComposeService};

impl ComposeService {
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
}
