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

    pub(super) fn title(self) -> &'static str {
        match self {
            Self::Deploy => "Compose deploy",
            Self::Redeploy => "Compose redeploy",
            Self::Reload => "Compose reload",
            Self::Start => "Compose start",
            Self::Stop => "Compose stop",
        }
    }

    pub(super) fn target_status(self) -> &'static str {
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
