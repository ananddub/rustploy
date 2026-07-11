use sqlx::SqlitePool;
use uuid::Uuid;

use super::ApplicationRecord;

pub(super) async fn select_application_by_id(
    db: &SqlitePool,
    id: i64,
) -> sqlx::Result<ApplicationRecord> {
    sqlx::query_as!(
        ApplicationRecord,
        r#"SELECT id AS "id!: i64", name, app_name, description, source_type, build_type, app_status, trigger_type,
           environment_id, server_id, build_server_id, registry_id, env_var, icon,
           repository, owner, branch, gitlab_repository, gitlab_owner, gitlab_branch,
           gitea_repository, gitea_owner, gitea_branch, bitbucket_repository, bitbucket_owner,
           bitbucket_branch, docker_image, registry_url, custom_git_url, custom_git_branch,
           created_at, updated_at
           FROM applications WHERE id = ?"#,
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
    let base = if slug.is_empty() { "app" } else { slug };
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}", base, &suffix[..6])
}
