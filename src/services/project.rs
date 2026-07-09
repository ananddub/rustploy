use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::{
    api::dto::project::{CreateProjectDto, PatchProjectDto},
    db::models::projects::Project,
};

pub struct ProjectService {
    db: Arc<SqlitePool>,
}
#[singleton]
impl ProjectService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Project> {
        sqlx::query_as!(
            Project,
            r#"SELECT id AS "id?", name, description, env_var, organization_id, created_at, updated_at
               FROM projects WHERE id = ?"#,
            id
        )
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn list_by_organization(&self, organization_id: i64) -> sqlx::Result<Vec<Project>> {
        sqlx::query_as!(
            Project,
            r#"SELECT id AS "id?", name, description, env_var, organization_id, created_at, updated_at
               FROM projects WHERE organization_id = ? ORDER BY created_at DESC, id DESC"#,
            organization_id
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn create(&self, input: CreateProjectDto) -> sqlx::Result<Project> {
        let mut tx = self.db.begin().await?;
        let project = sqlx::query_as!(
            Project,
            r#"INSERT INTO projects (name, description, env_var, organization_id) VALUES (?, ?, ?, ?)
               RETURNING id AS "id?", name, description, env_var, organization_id, created_at, updated_at"#,
            input.name,
            input.description,
            input.env_var,
            input.organization_id
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            "INSERT INTO environments (name, description, env_var, is_default, project_id) VALUES ('production', 'Production environment', '', 1, ?)",
            project.id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(project)
    }

    pub async fn update(&self, id: i64, input: PatchProjectDto) -> sqlx::Result<Project> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let env_var = input.env_var.unwrap_or(current.env_var);

        sqlx::query_as!(
            Project,
            r#"UPDATE projects SET name = ?, description = ?, env_var = ? WHERE id = ?
               RETURNING id AS "id?", name, description, env_var, organization_id, created_at, updated_at"#,
            name,
            description,
            env_var,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM projects WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    #[tokio::test]
    async fn create_also_creates_production_default_environment() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE organization (id INTEGER PRIMARY KEY)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE projects (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE, description TEXT, env_var TEXT NOT NULL DEFAULT '', organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE environments (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, description TEXT, env_var TEXT NOT NULL DEFAULT '', is_default INTEGER NOT NULL DEFAULT 0, project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO organization (id) VALUES (1)")
            .execute(&pool)
            .await
            .unwrap();
        let service = ProjectService { db: Arc::new(pool) };

        let project = service
            .create(CreateProjectDto {
                name: "website".into(),
                description: None,
                env_var: String::new(),
                organization_id: 1,
            })
            .await
            .unwrap();

        let environment: (String, i64) =
            sqlx::query_as("SELECT name, is_default FROM environments WHERE project_id = ?")
                .bind(project.id)
                .fetch_one(service.db.as_ref())
                .await
                .unwrap();
        assert_eq!(environment, ("production".into(), 1));
    }
}
