use std::{str::FromStr, sync::Arc};

use auto_di::singleton;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::api::dto::database::{CreateDatabaseDto, PatchDatabaseDto};

#[derive(Debug, Clone, Copy)]
pub enum DatabaseKind {
    Postgres,
    Mysql,
    Mariadb,
    Mongo,
    Redis,
    Libsql,
}

impl DatabaseKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Postgres => "postgres",
            Self::Mysql => "mysql",
            Self::Mariadb => "mariadb",
            Self::Mongo => "mongo",
            Self::Redis => "redis",
            Self::Libsql => "libsql",
        }
    }

    fn default_image(self) -> &'static str {
        match self {
            Self::Postgres => "postgres:18",
            Self::Mysql => "mysql:9",
            Self::Mariadb => "mariadb:13",
            Self::Mongo => "mongo:8",
            Self::Redis => "redis:8",
            Self::Libsql => "ghcr.io/tursodatabase/libsql-server:latest",
        }
    }
}

impl FromStr for DatabaseKind {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "postgres" | "postgresql" => Ok(Self::Postgres),
            "mysql" => Ok(Self::Mysql),
            "mariadb" => Ok(Self::Mariadb),
            "mongo" | "mongodb" => Ok(Self::Mongo),
            "redis" => Ok(Self::Redis),
            "libsql" => Ok(Self::Libsql),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseRecord {
    pub kind: String,
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub docker_image: String,
    pub database_name: Option<String>,
    pub database_user: Option<String>,
    pub external_port: Option<i64>,
    pub env_var: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: i64,
    pub app_status: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum DatabaseOperation {
    Deploy,
    Redeploy,
    Reload,
    Start,
    Stop,
}

impl DatabaseOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deploy => "deploy",
            Self::Redeploy => "redeploy",
            Self::Reload => "reload",
            Self::Start => "start",
            Self::Stop => "stop",
        }
    }

    fn target_status(self) -> &'static str {
        match self {
            Self::Stop => "IDLE",
            _ => "RUNNING",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseOperationResult {
    pub database: DatabaseRecord,
    pub operation: DatabaseOperation,
}

pub struct DatabaseService {
    db: Arc<SqlitePool>,
}

macro_rules! patch_common {
    ($self:ident, $sql:literal, $id:ident, $input:ident) => {
        sqlx::query!(
            $sql,
            $input.name,
            $input.description,
            $input.docker_image,
            $input.external_port,
            $input.command,
            $input.args,
            $input.env_var,
            $input.memory_reservation,
            $input.memory_limit,
            $input.cpu_reservation,
            $input.cpu_limit,
            $input.replicas,
            $input.server_id,
            $id
        )
        .execute($self.db.as_ref())
        .await
    };
}

macro_rules! update_status {
    ($self:ident, $sql:literal, $id:ident, $operation:ident) => {
        sqlx::query!($sql, $operation.target_status(), $id)
            .execute($self.db.as_ref())
            .await
    };
}

#[singleton]
impl DatabaseService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<DatabaseRecord>> {
        sqlx::query_as!(
            DatabaseRecord,
            r#"SELECT 'postgres' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
               database_name AS "database_name?", database_user AS "database_user?", external_port,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               app_status, environment_id, server_id, created_at, updated_at
               FROM postgres_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'mysql' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
               database_name AS "database_name?", database_user AS "database_user?", external_port,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               app_status, environment_id, server_id, created_at, updated_at
               FROM mysql_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'mariadb' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
               database_name AS "database_name?", database_user AS "database_user?", external_port,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               app_status, environment_id, server_id, created_at, updated_at
               FROM mariadb_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'mongo' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS "database_name?", database_user AS "database_user?", external_port,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               app_status, environment_id, server_id, created_at, updated_at
               FROM mongo_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'redis' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS "database_name?", CAST(NULL AS TEXT) AS "database_user?", external_port,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               app_status, environment_id, server_id, created_at, updated_at
               FROM redis_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'libsql' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS "database_name?", database_user AS "database_user?", external_port,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               app_status, environment_id, server_id, created_at, updated_at
               FROM libsql_dbs WHERE environment_id = ?
               ORDER BY created_at DESC, id DESC"#,
            environment_id,
            environment_id,
            environment_id,
            environment_id,
            environment_id,
            environment_id
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn get_by_id(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<DatabaseRecord> {
        match kind {
            DatabaseKind::Postgres => select_postgres(self.db.as_ref(), id).await,
            DatabaseKind::Mysql => select_mysql(self.db.as_ref(), id).await,
            DatabaseKind::Mariadb => select_mariadb(self.db.as_ref(), id).await,
            DatabaseKind::Mongo => select_mongo(self.db.as_ref(), id).await,
            DatabaseKind::Redis => select_redis(self.db.as_ref(), id).await,
            DatabaseKind::Libsql => select_libsql(self.db.as_ref(), id).await,
        }
    }

    pub async fn create(
        &self,
        kind: DatabaseKind,
        input: CreateDatabaseDto,
    ) -> sqlx::Result<DatabaseRecord> {
        let app_name = generate_app_name(&input.name, kind.as_str());
        let image = input
            .docker_image
            .unwrap_or_else(|| kind.default_image().into());
        let db_name = input
            .database_name
            .clone()
            .unwrap_or_else(|| slug_value(&input.name));
        let db_user = input
            .database_user
            .clone()
            .unwrap_or_else(|| "rustploy".into());
        let db_password = input
            .database_password
            .clone()
            .unwrap_or_else(random_secret);
        let root_password = input
            .database_root_password
            .clone()
            .unwrap_or_else(random_secret);

        match kind {
            DatabaseKind::Postgres => {
                sqlx::query!(
                    r#"INSERT INTO postgres_dbs
                       (name, app_name, description, docker_image, database_name, database_user,
                        database_password, external_port, environment_id, server_id)
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
                    input.name,
                    app_name,
                    input.description,
                    image,
                    db_name,
                    db_user,
                    db_password,
                    input.external_port,
                    input.environment_id,
                    input.server_id
                )
                .execute(self.db.as_ref())
                .await?;
            }
            DatabaseKind::Mysql => {
                sqlx::query!(
                    r#"INSERT INTO mysql_dbs
                       (name, app_name, description, docker_image, database_name, database_user,
                        database_password, database_root_password, external_port, environment_id, server_id)
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
                    input.name,
                    app_name,
                    input.description,
                    image,
                    db_name,
                    db_user,
                    db_password,
                    root_password,
                    input.external_port,
                    input.environment_id,
                    input.server_id
                )
                .execute(self.db.as_ref())
                .await?;
            }
            DatabaseKind::Mariadb => {
                sqlx::query!(
                    r#"INSERT INTO mariadb_dbs
                       (name, app_name, description, docker_image, database_name, database_user,
                        database_password, database_root_password, external_port, environment_id, server_id)
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
                    input.name,
                    app_name,
                    input.description,
                    image,
                    db_name,
                    db_user,
                    db_password,
                    root_password,
                    input.external_port,
                    input.environment_id,
                    input.server_id
                )
                .execute(self.db.as_ref())
                .await?;
            }
            DatabaseKind::Mongo => {
                sqlx::query!(
                    r#"INSERT INTO mongo_dbs
                       (name, app_name, description, docker_image, database_user, database_password,
                        external_port, replica_sets, environment_id, server_id)
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
                    input.name,
                    app_name,
                    input.description,
                    image,
                    db_user,
                    db_password,
                    input.external_port,
                    input.replica_sets.unwrap_or(0),
                    input.environment_id,
                    input.server_id
                )
                .execute(self.db.as_ref())
                .await?;
            }
            DatabaseKind::Redis => {
                sqlx::query!(
                    r#"INSERT INTO redis_dbs
                       (name, app_name, description, docker_image, database_password,
                        external_port, environment_id, server_id)
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
                    input.name,
                    app_name,
                    input.description,
                    image,
                    db_password,
                    input.external_port,
                    input.environment_id,
                    input.server_id
                )
                .execute(self.db.as_ref())
                .await?;
            }
            DatabaseKind::Libsql => {
                sqlx::query!(
                    r#"INSERT INTO libsql_dbs
                       (name, app_name, description, docker_image, database_user, database_password,
                        sqld_node, sqld_primary_url, enable_namespaces, external_port, environment_id, server_id)
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
                    input.name,
                    app_name,
                    input.description,
                    image,
                    db_user,
                    db_password,
                    input.sqld_node.unwrap_or_else(|| "PRIMARY".into()),
                    input.sqld_primary_url,
                    input.enable_namespaces.unwrap_or(0),
                    input.external_port,
                    input.environment_id,
                    input.server_id
                )
                .execute(self.db.as_ref())
                .await?;
            }
        }

        let id = sqlx::query_scalar!("SELECT last_insert_rowid() AS \"id!: i64\"")
            .fetch_one(self.db.as_ref())
            .await?;
        self.get_by_id(kind, id).await
    }

    pub async fn patch(
        &self,
        kind: DatabaseKind,
        id: i64,
        input: PatchDatabaseDto,
    ) -> sqlx::Result<DatabaseRecord> {
        match kind {
            DatabaseKind::Postgres => patch_common!(
                self,
                "UPDATE postgres_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id) WHERE id = ?",
                id,
                input
            ),
            DatabaseKind::Mysql => patch_common!(
                self,
                "UPDATE mysql_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id) WHERE id = ?",
                id,
                input
            ),
            DatabaseKind::Mariadb => patch_common!(
                self,
                "UPDATE mariadb_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id) WHERE id = ?",
                id,
                input
            ),
            DatabaseKind::Mongo => patch_common!(
                self,
                "UPDATE mongo_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id) WHERE id = ?",
                id,
                input
            ),
            DatabaseKind::Redis => patch_common!(
                self,
                "UPDATE redis_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id) WHERE id = ?",
                id,
                input
            ),
            DatabaseKind::Libsql => patch_common!(
                self,
                "UPDATE libsql_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id) WHERE id = ?",
                id,
                input
            ),
        }?;

        self.get_by_id(kind, id).await
    }

    pub async fn run_operation(
        &self,
        kind: DatabaseKind,
        id: i64,
        operation: DatabaseOperation,
    ) -> sqlx::Result<DatabaseOperationResult> {
        match kind {
            DatabaseKind::Postgres => {
                update_status!(
                    self,
                    "UPDATE postgres_dbs SET app_status = ? WHERE id = ?",
                    id,
                    operation
                )
            }
            DatabaseKind::Mysql => {
                update_status!(
                    self,
                    "UPDATE mysql_dbs SET app_status = ? WHERE id = ?",
                    id,
                    operation
                )
            }
            DatabaseKind::Mariadb => {
                update_status!(
                    self,
                    "UPDATE mariadb_dbs SET app_status = ? WHERE id = ?",
                    id,
                    operation
                )
            }
            DatabaseKind::Mongo => {
                update_status!(
                    self,
                    "UPDATE mongo_dbs SET app_status = ? WHERE id = ?",
                    id,
                    operation
                )
            }
            DatabaseKind::Redis => {
                update_status!(
                    self,
                    "UPDATE redis_dbs SET app_status = ? WHERE id = ?",
                    id,
                    operation
                )
            }
            DatabaseKind::Libsql => {
                update_status!(
                    self,
                    "UPDATE libsql_dbs SET app_status = ? WHERE id = ?",
                    id,
                    operation
                )
            }
        }?;

        Ok(DatabaseOperationResult {
            database: self.get_by_id(kind, id).await?,
            operation,
        })
    }

    pub async fn delete(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<()> {
        self.get_by_id(kind, id).await?;
        match kind {
            DatabaseKind::Postgres => {
                sqlx::query!("DELETE FROM postgres_dbs WHERE id = ?", id)
                    .execute(self.db.as_ref())
                    .await?;
            }
            DatabaseKind::Mysql => {
                sqlx::query!("DELETE FROM mysql_dbs WHERE id = ?", id)
                    .execute(self.db.as_ref())
                    .await?;
            }
            DatabaseKind::Mariadb => {
                sqlx::query!("DELETE FROM mariadb_dbs WHERE id = ?", id)
                    .execute(self.db.as_ref())
                    .await?;
            }
            DatabaseKind::Mongo => {
                sqlx::query!("DELETE FROM mongo_dbs WHERE id = ?", id)
                    .execute(self.db.as_ref())
                    .await?;
            }
            DatabaseKind::Redis => {
                sqlx::query!("DELETE FROM redis_dbs WHERE id = ?", id)
                    .execute(self.db.as_ref())
                    .await?;
            }
            DatabaseKind::Libsql => {
                sqlx::query!("DELETE FROM libsql_dbs WHERE id = ?", id)
                    .execute(self.db.as_ref())
                    .await?;
            }
        }
        Ok(())
    }
}

async fn select_postgres(db: &SqlitePool, id: i64) -> sqlx::Result<DatabaseRecord> {
    sqlx::query_as!(
        DatabaseRecord,
        r#"SELECT 'postgres' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
           database_name AS "database_name?", database_user AS "database_user?", external_port,
           env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
           app_status, environment_id, server_id, created_at, updated_at
           FROM postgres_dbs WHERE id = ?"#,
        id
    )
    .fetch_one(db)
    .await
}

async fn select_mysql(db: &SqlitePool, id: i64) -> sqlx::Result<DatabaseRecord> {
    sqlx::query_as!(
        DatabaseRecord,
        r#"SELECT 'mysql' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
           database_name AS "database_name?", database_user AS "database_user?", external_port,
           env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
           app_status, environment_id, server_id, created_at, updated_at
           FROM mysql_dbs WHERE id = ?"#,
        id
    )
    .fetch_one(db)
    .await
}

async fn select_mariadb(db: &SqlitePool, id: i64) -> sqlx::Result<DatabaseRecord> {
    sqlx::query_as!(
        DatabaseRecord,
        r#"SELECT 'mariadb' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
           database_name AS "database_name?", database_user AS "database_user?", external_port,
           env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
           app_status, environment_id, server_id, created_at, updated_at
           FROM mariadb_dbs WHERE id = ?"#,
        id
    )
    .fetch_one(db)
    .await
}

async fn select_mongo(db: &SqlitePool, id: i64) -> sqlx::Result<DatabaseRecord> {
    sqlx::query_as!(
        DatabaseRecord,
        r#"SELECT 'mongo' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
           CAST(NULL AS TEXT) AS "database_name?", database_user AS "database_user?", external_port,
           env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
           app_status, environment_id, server_id, created_at, updated_at
           FROM mongo_dbs WHERE id = ?"#,
        id
    )
    .fetch_one(db)
    .await
}

async fn select_redis(db: &SqlitePool, id: i64) -> sqlx::Result<DatabaseRecord> {
    sqlx::query_as!(
        DatabaseRecord,
        r#"SELECT 'redis' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
           CAST(NULL AS TEXT) AS "database_name?", CAST(NULL AS TEXT) AS "database_user?", external_port,
           env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
           app_status, environment_id, server_id, created_at, updated_at
           FROM redis_dbs WHERE id = ?"#,
        id
    )
    .fetch_one(db)
    .await
}

async fn select_libsql(db: &SqlitePool, id: i64) -> sqlx::Result<DatabaseRecord> {
    sqlx::query_as!(
        DatabaseRecord,
        r#"SELECT 'libsql' AS kind, id AS "id!: i64", name, app_name, description, docker_image,
           CAST(NULL AS TEXT) AS "database_name?", database_user AS "database_user?", external_port,
           env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
           app_status, environment_id, server_id, created_at, updated_at
           FROM libsql_dbs WHERE id = ?"#,
        id
    )
    .fetch_one(db)
    .await
}

fn generate_app_name(name: &str, prefix: &str) -> String {
    let slug = slug_value(name);
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}-{}", prefix, slug, &suffix[..6])
}

fn slug_value(name: &str) -> String {
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
    if slug.is_empty() {
        "database".into()
    } else {
        slug.into()
    }
}

fn random_secret() -> String {
    Uuid::new_v4().simple().to_string()
}
