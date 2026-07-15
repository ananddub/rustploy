use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::api::dto::domain::{CreateDomainDto, PatchDomainDto};
use crate::services::compose::ComposeType;
use crate::utils::builder::application::traefik::build_traefik_labels;
use crate::utils::builder::compose::labels::build_compose_service_labels;
use crate::utils::builder::spec::{ApplicationSpec, DomainSpec, ResourceSpec, SourceSpec};
use crate::utils::docker::DockerCli;
use crate::utils::exec::{CommandExecutor, LocalExecutor};

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
        let domain_type = match (input.application_id, input.compose_id) {
            (Some(_), None) => "APPLICATION",
            (None, Some(_)) => "COMPOSE",
            _ => "APPLICATION",
        };

        let domain = sqlx::query_as!(
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
        .await?;

        if let Some(app_id) = domain.application_id.filter(|&id| id > 0) {
            self.apply_application_traefik(app_id).await;
        }
        if let Some(compose_id) = domain.compose_id.filter(|&id| id > 0) {
            self.apply_compose_traefik(compose_id).await;
        }

        Ok(domain)
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
        let strip_path = input.strip_path.map(bool_to_i64).unwrap_or(current.strip_path);
        let middlewares = input.middlewares.unwrap_or(current.middlewares);
        let certificate_type = input
            .certificate_type
            .map(|v| v.to_uppercase())
            .unwrap_or(current.certificate_type);

        let domain = sqlx::query_as!(
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
        .await?;

        // Auto-apply traefik labels after domain update.
        if let Some(app_id) = domain.application_id.filter(|&id| id > 0) {
            self.apply_application_traefik(app_id).await;
        }
        if let Some(compose_id) = domain.compose_id.filter(|&id| id > 0) {
            self.apply_compose_traefik(compose_id).await;
        }

        Ok(domain)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        let domain = self.get_by_id(id).await?;
        let result = sqlx::query!("DELETE FROM domains WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        if let Some(app_id) = domain.application_id.filter(|&id| id > 0) {
            self.apply_application_traefik(app_id).await;
        }
        if let Some(compose_id) = domain.compose_id.filter(|&id| id > 0) {
            self.apply_compose_traefik(compose_id).await;
        }

        Ok(())
    }


    async fn apply_application_traefik(&self, application_id: i64) {
        if let Err(e) = self.try_apply_application_traefik(application_id).await {
            tracing::warn!(
                application_id,
                error = %e,
                "could not apply traefik labels after domain change"
            );
        }
    }

    async fn try_apply_application_traefik(&self, application_id: i64) -> Result<(), String> {
        let (app_name, server_id) = sqlx::query_as::<_, (String, Option<i64>)>(
            "SELECT app_name, server_id FROM applications WHERE id = ?",
        )
        .bind(application_id)
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| format!("app not found: {e}"))?;

        let service_name = format!("{app_name}_{app_name}");

        // Load all remaining domains for this application.
        let domains = self
            .list_by_application(application_id)
            .await
            .map_err(|e| format!("could not load domains: {e}"))?;

        // Build minimal ApplicationSpec just for label generation.
        let spec = ApplicationSpec {
            app_name: app_name.clone(),
            stack_name: app_name.clone(),
            source: SourceSpec::Docker { image: String::new(), registry: None },
            build: None,
            work_directory: String::new(),
            image: String::new(),
            environment: Default::default(),
            build_args: Default::default(),
            build_secrets: Default::default(),
            command: None,
            args: vec![],
            replicas: 1,
            network: "rustploy-network".into(),
            mounts: vec![],
            domains: domains
                .iter()
                .map(|d| DomainSpec {
                    key: d.id.to_string(),
                    host: d.host.clone(),
                    https: d.https != 0,
                    port: d.port.unwrap_or(3000) as u16,
                    service_name: d.service_name.clone(),
                    path: d.path.clone().unwrap_or_else(|| "/".into()),
                    internal_path: d.internal_path.clone().unwrap_or_else(|| "/".into()),
                    strip_path: d.strip_path != 0,
                    entrypoint: d.custom_entrypoint.clone(),
                    certificate_type: d.certificate_type.clone(),
                    custom_cert_resolver: d.custom_cert_resolver.clone(),
                    middlewares: serde_json::from_str(&d.middlewares).unwrap_or_default(),
                })
                .collect(),
            resources: ResourceSpec::default(),
            healthcheck: None,
            placement_constraints: vec![],
            stop_grace_period: None,
        };

        let new_labels = build_traefik_labels(&spec);

        // Build docker executor (local or remote).
        let executor = match server_id {
            Some(sid) => {
                use crate::services::application::remote::remote_executor;
                let remote = remote_executor(self.db.as_ref(), sid)
                    .await
                    .map_err(|e| format!("remote executor error: {e}"))?;
                CommandExecutor::Remote(remote)
            }
            None => CommandExecutor::Local(LocalExecutor::new()),
        };

        let docker = DockerCli::from_executor(executor);

        // Get current traefik labels to remove stale ones.
        let inspect_out = docker
            .run(["service", "inspect", "--format", "{{json .Spec.Labels}}", &service_name])
            .await
            .map_err(|e| format!("service inspect failed: {e}"))?;

        let current: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(inspect_out.stdout_trimmed()).unwrap_or_default();

        let stale: Vec<String> = current
            .keys()
            .filter(|k| k.starts_with("traefik."))
            .cloned()
            .collect();

        let mut args: Vec<String> = vec![];
        for key in &stale {
            args.push("--label-rm".into());
            args.push(key.clone());
        }
        for label in &new_labels {
            args.push("--label-add".into());
            args.push(label.clone());
        }

        if args.is_empty() {
            return Ok(());
        }

        args.push(service_name.clone());
        let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        docker
            .service_update(&refs)
            .await
            .map_err(|e| format!("service update failed: {e}"))?;

        tracing::info!(
            application_id,
            service = %service_name,
            labels = new_labels.len(),
            "traefik labels updated after domain change"
        );

        Ok(())
    }

    async fn apply_compose_traefik(&self, compose_id: i64) {
        if let Err(e) = self.try_apply_compose_traefik(compose_id).await {
            tracing::warn!(
                compose_id,
                error = %e,
                "could not apply traefik labels after compose domain change"
            );
        }
    }

    async fn try_apply_compose_traefik(&self, compose_id: i64) -> Result<(), String> {
        let (app_name, server_id, compose_type) =
            sqlx::query_as::<_, (String, Option<i64>, String)>(
                "SELECT app_name, server_id, compose_type FROM compose_projects WHERE id = ?",
            )
            .bind(compose_id)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(|e| format!("compose not found: {e}"))?;

        let executor = match server_id {
            Some(sid) => {
                use crate::services::compose::remote::remote_executor;
                let remote = remote_executor(self.db.as_ref(), sid)
                    .await
                    .map_err(|e| format!("remote executor: {e}"))?;
                CommandExecutor::Remote(remote)
            }
            None => CommandExecutor::Local(LocalExecutor::new()),
        };
        let docker = DockerCli::from_executor(executor.clone());

        let domains = self
            .list_by_compose(compose_id)
            .await
            .map_err(|e| format!("could not load domains: {e}"))?;
        let v = ComposeType::try_from(compose_type.as_str()).map_err(|e| format!("invalid compose type: {e}"))?;
        match v {
            ComposeType::Stack => {
                let labels = build_compose_service_labels(&app_name, &domains);
                for (service_name, service_labels) in labels {
                    // Remove stale traefik labels first.
                    let inspect_out = docker
                        .run(["service", "inspect", "--format", "{{json .Spec.Labels}}", &service_name])
                        .await
                        .map_err(|e| format!("service inspect failed: {e}"))?;
                    let current: serde_json::Map<String, serde_json::Value> =
                        serde_json::from_str(inspect_out.stdout_trimmed()).unwrap_or_default();
                    let stale: Vec<String> = current.keys()
                        .filter(|k| k.starts_with("traefik."))
                        .cloned()
                        .collect();

                    let mut args: Vec<String> = vec![];
                    for key in &stale {
                        args.push("--label-rm".into());
                        args.push(key.clone());
                    }
                    for label in &service_labels {
                        args.push("--label-add".into());
                        args.push(label.clone());
                    }
                    if args.is_empty() { continue; }
                    args.push(service_name.clone());
                    let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                    docker.service_update(&refs).await
                        .map_err(|e| format!("service update labels failed: {e}"))?;

                    tracing::info!(compose_id, service = %service_name, "traefik labels updated for compose stack service");
                }
            }
            ComposeType::DockerCompose => {
                use crate::utils::paths::rustploy_paths;
                let paths = rustploy_paths();
                let env_file = format!("{}/{app_name}/.env", paths.compose_dir("").trim_end_matches('/'));
                let compose_file = format!("{}/{app_name}/source/docker-compose.yml", paths.compose_dir("").trim_end_matches('/'));
                executor.run("docker", [
                    "compose",
                    "--project-name", &app_name,
                    "--env-file", &env_file,
                    "--file", &compose_file,
                    "up", "--detach", "--no-build",
                ]).await.map_err(|e| format!("compose up failed: {e}"))?;
                tracing::info!(compose_id, "traefik labels updated via compose up");
            }
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
