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
        self.repo_compose.set_github_source(
            id,
            Some(input.repository),
            Some(input.owner),
            Some(input.branch),
            input.github_provider_id,
            input.auto_deploy.unwrap_or(1),
        )
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_gitlab_source(
        &self,
        id: i64,
        input: PatchComposeGitlabSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose.set_gitlab_source(
            id,
            input.gitlab_project_id,
            Some(input.gitlab_repository),
            Some(input.gitlab_owner),
            Some(input.gitlab_branch),
            input.gitlab_path_namespace,
            input.gitlab_provider_id,
        )
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_gitea_source(
        &self,
        id: i64,
        input: PatchComposeGiteaSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose.set_gitea_source(
            id,
            Some(input.gitea_repository),
            Some(input.gitea_owner),
            Some(input.gitea_branch),
            input.gitea_provider_id,
        )
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_bitbucket_source(
        &self,
        id: i64,
        input: PatchComposeBitbucketSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose.set_bitbucket_source(
            id,
            Some(input.bitbucket_repository),
            input.bitbucket_repository_slug,
            Some(input.bitbucket_owner),
            Some(input.bitbucket_branch),
            input.bitbucket_provider_id,
        )
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_custom_git_source(
        &self,
        id: i64,
        input: PatchComposeCustomGitSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose.set_custom_git_source(
            id,
            Some(input.custom_git_url),
            Some(input.custom_git_branch),
            input.custom_git_ssh_key_id,
        )
        .await?;
        self.get_by_id(id).await
    }

    pub async fn set_raw_source(
        &self,
        id: i64,
        input: PatchComposeRawSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose.set_raw_source(
            id,
            input.compose_file,
        )
        .await?;
        self.get_by_id(id).await
    }
}
