use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::api::dto::domain::{CreateDomainDto, PatchDomainDto};

#[derive(Debug, Clone)]
pub struct DomainRecord {
    pub id: i64,
    pub host: String,
    pub https: i64,
    pub port: Option<i64>,
    pub path: Option<String>,
    pub internal_path: Option<String>,
    pub custom_entrypoint: Option<String>,
    pub service_name: Option<String>,
    pub custom_cert_resolver: Option<String>,
    pub strip_path: i64,
    pub middlewares: String,
    pub domain_type: String,
    pub certificate_type: String,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct DomainService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl DomainService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<DomainRecord> {
        select_domain_by_id(self.db.as_ref(), id).await
    }

    pub async fn list_by_application(
        &self,
        application_id: i64,
    ) -> sqlx::Result<Vec<DomainRecord>> {
        sqlx::query_as!(
            DomainRecord,
            r#"SELECT id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at
               FROM domains
               WHERE application_id = ?
               ORDER BY created_at DESC, id DESC"#,
            application_id
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn list_by_compose(&self, compose_id: i64) -> sqlx::Result<Vec<DomainRecord>> {
        sqlx::query_as!(
            DomainRecord,
            r#"SELECT id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at
               FROM domains
               WHERE compose_id = ?
               ORDER BY created_at DESC, id DESC"#,
            compose_id
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn create(&self, input: CreateDomainDto) -> sqlx::Result<DomainRecord> {
        let https = bool_to_i64(input.https);
        let strip_path = bool_to_i64(input.strip_path);
        let port = input.port.or(Some(3000));
        let certificate_type = input.certificate_type.to_uppercase();
        // Auto-detect domain_type from which FK is set — don't trust frontend value.
        let domain_type = match (input.application_id, input.compose_id) {
            (Some(_), None) => "APPLICATION",
            (None, Some(_)) => "COMPOSE",
            _ => "APPLICATION",
        };

        sqlx::query_as!(
            DomainRecord,
            r#"INSERT INTO domains
               (host, https, port, path, internal_path, custom_entrypoint, service_name,
                custom_cert_resolver, strip_path, middlewares, domain_type, certificate_type,
                application_id, compose_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at"#,
            input.host,
            https,
            port,
            input.path,
            input.internal_path,
            input.custom_entrypoint,
            input.service_name,
            input.custom_cert_resolver,
            strip_path,
            input.middlewares,
            domain_type,
            certificate_type,
            input.application_id,
            input.compose_id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn patch(&self, id: i64, input: PatchDomainDto) -> sqlx::Result<DomainRecord> {
        let current = self.get_by_id(id).await?;
        let host = input.host.unwrap_or(current.host);
        let https = input.https.map(bool_to_i64).unwrap_or(current.https);
        let port = input.port.or(current.port);
        let path = input.path.or(current.path);
        let internal_path = input.internal_path.or(current.internal_path);
        let custom_entrypoint = input.custom_entrypoint.or(current.custom_entrypoint);
        let service_name = input.service_name.or(current.service_name);
        let custom_cert_resolver = input.custom_cert_resolver.or(current.custom_cert_resolver);
        let strip_path = input
            .strip_path
            .map(bool_to_i64)
            .unwrap_or(current.strip_path);
        let middlewares = input.middlewares.unwrap_or(current.middlewares);
        let certificate_type = input.certificate_type
            .map(|v| v.to_uppercase())
            .unwrap_or(current.certificate_type);

        sqlx::query_as!(
            DomainRecord,
            r#"UPDATE domains SET
               host = ?, https = ?, port = ?, path = ?, internal_path = ?,
               custom_entrypoint = ?, service_name = ?, custom_cert_resolver = ?,
               strip_path = ?, middlewares = ?, certificate_type = ?
               WHERE id = ?
               RETURNING id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at"#,
            host,
            https,
            port,
            path,
            internal_path,
            custom_entrypoint,
            service_name,
            custom_cert_resolver,
            strip_path,
            middlewares,
            certificate_type,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        let result = sqlx::query!("DELETE FROM domains WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
}

fn bool_to_i64(value: bool) -> i64 {
    if value { 1 } else { 0 }
}

async fn select_domain_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<DomainRecord> {
    sqlx::query_as!(
        DomainRecord,
        r#"SELECT id AS "id!: i64", host, https, port, path, internal_path,
           custom_entrypoint, service_name, custom_cert_resolver, strip_path,
           middlewares, domain_type, certificate_type, application_id, compose_id,
           created_at, updated_at
           FROM domains
           WHERE id = ?"#,
        id
    )
    .fetch_one(pool)
    .await
}
