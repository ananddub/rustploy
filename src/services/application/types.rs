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

    pub(super) fn title(self) -> &'static str {
        match self {
            Self::Deploy => "Application deploy",
            Self::Redeploy => "Application redeploy",
            Self::Rebuild => "Application rebuild",
            Self::Reload => "Application reload",
            Self::Start => "Application start",
        }
    }

    pub(super) fn target_status(self) -> &'static str {
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
