use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::{
    api::dto::organization::{CreateOrganizationDto, PatchOrganizationDto},
    db::models::organization::Organization,
};

pub struct OrganizationService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl OrganizationService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Organization> {
        sqlx::query_as!(
            Organization,
            r#"SELECT id AS "id?", name, logo, slug, owner_id, created_at, updated_at
               FROM organization WHERE id = ?"#,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn list_by_owner(&self, owner_id: i64) -> sqlx::Result<Vec<Organization>> {
        sqlx::query_as!(
            Organization,
            r#"SELECT id AS "id?", name, logo, slug, owner_id, created_at, updated_at
               FROM organization WHERE owner_id = ? ORDER BY created_at DESC, id DESC"#,
            owner_id
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn create(&self, input: CreateOrganizationDto) -> sqlx::Result<Organization> {
        let slug = normalize_slug(input.slug.as_deref().unwrap_or(&input.name));
        if slug.is_empty() {
            return Err(sqlx::Error::Protocol(
                "organization slug cannot be empty".into(),
            ));
        }

        let mut tx = self.db.begin().await?;
        let organization = sqlx::query_as!(
            Organization,
            r#"INSERT INTO organization (name, logo, slug, owner_id) VALUES (?, ?, ?, ?)
               RETURNING id AS "id?", name, logo, slug, owner_id, created_at, updated_at"#,
            input.name,
            input.logo,
            slug,
            input.owner_id
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            "INSERT INTO organization_members (role, user_id, organization_id) VALUES ('ADMIN', ?, ?)",
            input.owner_id,
            organization.id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(organization)
    }

    pub async fn update(&self, id: i64, input: PatchOrganizationDto) -> sqlx::Result<Organization> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let logo = input.logo.or(current.logo);
        let slug = input
            .slug
            .map(|value| normalize_slug(&value))
            .unwrap_or(current.slug);
        if slug.is_empty() {
            return Err(sqlx::Error::Protocol(
                "organization slug cannot be empty".into(),
            ));
        }

        sqlx::query_as!(
            Organization,
            r#"UPDATE organization SET name = ?, logo = ?, slug = ? WHERE id = ?
               RETURNING id AS "id?", name, logo, slug, owner_id, created_at, updated_at"#,
            name,
            logo,
            slug,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM organization WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}

fn normalize_slug(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .fold((String::new(), false), |(mut output, separator), ch| {
            if ch.is_ascii_alphanumeric() {
                output.push(ch);
                (output, false)
            } else if !output.is_empty() && !separator {
                output.push('-');
                (output, true)
            } else {
                (output, separator)
            }
        })
        .0
        .trim_end_matches('-')
        .to_owned()
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    #[tokio::test]
    async fn create_normalizes_slug_and_adds_owner_as_admin() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE organization (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE, logo TEXT, slug TEXT NOT NULL UNIQUE, owner_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE organization_members (id INTEGER PRIMARY KEY AUTOINCREMENT, role TEXT DEFAULT 'MEMBER', user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO users (id) VALUES (7)")
            .execute(&pool)
            .await
            .unwrap();
        let service = OrganizationService { db: Arc::new(pool) };

        let organization = service
            .create(CreateOrganizationDto {
                name: "My Cool Team".into(),
                logo: None,
                slug: None,
                owner_id: 7,
            })
            .await
            .unwrap();

        assert_eq!(organization.slug, "my-cool-team");
        let role: String = sqlx::query_scalar(
            "SELECT role FROM organization_members WHERE organization_id = ? AND user_id = 7",
        )
        .bind(organization.id)
        .fetch_one(service.db.as_ref())
        .await
        .unwrap();
        assert_eq!(role, "ADMIN");
    }
}
