use crate::db::models::domains::Domain;
use crate::db::models::types::*;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use std::sync::Arc;
use auto_di::singleton;

pub struct DomainRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl DomainRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Domain>, sqlx::Error> {
        sqlx::query_as!(
            Domain,
            r#"SELECT id AS "id?: i64", host AS "host: String", https AS "https: i64", port AS "port?: i64", path AS "path?: String", internal_path AS "internal_path?: String", custom_entrypoint AS "custom_entrypoint?: String", service_name AS "service_name?: String", custom_cert_resolver AS "custom_cert_resolver?: String", strip_path AS "strip_path: i64", middlewares AS "middlewares: String", domain_type AS "domain_type: String", certificate_type AS "certificate_type: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM domains"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Domain>, sqlx::Error> {
        sqlx::query_as!(
            Domain,
            r#"SELECT id AS "id?: i64", host AS "host: String", https AS "https: i64", port AS "port?: i64", path AS "path?: String", internal_path AS "internal_path?: String", custom_entrypoint AS "custom_entrypoint?: String", service_name AS "service_name?: String", custom_cert_resolver AS "custom_cert_resolver?: String", strip_path AS "strip_path: i64", middlewares AS "middlewares: String", domain_type AS "domain_type: String", certificate_type AS "certificate_type: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM domains WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Domain) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO domains (host, https, port, path, internal_path, custom_entrypoint, service_name, custom_cert_resolver, strip_path, middlewares, domain_type, certificate_type, application_id, compose_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.host,
            item.https,
            item.port,
            &item.path,
            &item.internal_path,
            &item.custom_entrypoint,
            &item.service_name,
            &item.custom_cert_resolver,
            item.strip_path,
            &item.middlewares,
            &item.domain_type,
            &item.certificate_type,
            item.application_id,
            item.compose_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Domain) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE domains SET host = ?, https = ?, port = ?, path = ?, internal_path = ?, custom_entrypoint = ?, service_name = ?, custom_cert_resolver = ?, strip_path = ?, middlewares = ?, domain_type = ?, certificate_type = ?, application_id = ?, compose_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.host,
            item.https,
            item.port,
            &item.path,
            &item.internal_path,
            &item.custom_entrypoint,
            &item.service_name,
            &item.custom_cert_resolver,
            item.strip_path,
            &item.middlewares,
            &item.domain_type,
            &item.certificate_type,
            item.application_id,
            item.compose_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM domains WHERE id = ?"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
