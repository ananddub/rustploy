use sqlx::SqlitePool;
use uuid::Uuid;

use super::ComposeRecord;

pub(super) async fn select_compose_by_id(db: &SqlitePool, id: i64) -> sqlx::Result<ComposeRecord> {
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

pub(super) fn generate_app_name(name: &str) -> String {
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
