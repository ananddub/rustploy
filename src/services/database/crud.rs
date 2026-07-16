use crate::api::dto::database::{CreateDatabaseDto, PatchDatabaseDto};
use super::{
    DatabaseService, DatabaseRecord, DatabaseKind,
    queries::{generate_app_name, slug_value, random_secret},
};

impl DatabaseService {
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
            DatabaseKind::Postgres => self.select_postgres(id).await,
            DatabaseKind::Mysql => self.select_mysql(id).await,
            DatabaseKind::Mariadb => self.select_mariadb(id).await,
            DatabaseKind::Mongo => self.select_mongo(id).await,
            DatabaseKind::Redis => self.select_redis(id).await,
            DatabaseKind::Libsql => self.select_libsql(id).await,
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
            .clone()
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
                self.create_postgres(&input, &app_name, &image, &db_name, &db_user, &db_password).await?;
            }
            DatabaseKind::Mysql => {
                self.create_mysql(&input, &app_name, &image, &db_name, &db_user, &db_password, &root_password).await?;
            }
            DatabaseKind::Mariadb => {
                self.create_mariadb(&input, &app_name, &image, &db_name, &db_user, &db_password, &root_password).await?;
            }
            DatabaseKind::Mongo => {
                self.create_mongo(&input, &app_name, &image, &db_user, &db_password).await?;
            }
            DatabaseKind::Redis => {
                self.create_redis(&input, &app_name, &image, &db_password).await?;
            }
            DatabaseKind::Libsql => {
                self.create_libsql(&input, &app_name, &image, &db_user, &db_password).await?;
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
            DatabaseKind::Postgres => self.patch_postgres(id, &input).await?,
            DatabaseKind::Mysql => self.patch_mysql(id, &input).await?,
            DatabaseKind::Mariadb => self.patch_mariadb(id, &input).await?,
            DatabaseKind::Mongo => self.patch_mongo(id, &input).await?,
            DatabaseKind::Redis => self.patch_redis(id, &input).await?,
            DatabaseKind::Libsql => self.patch_libsql(id, &input).await?,
        }

        self.get_by_id(kind, id).await
    }

    pub async fn delete(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<()> {
        self.get_by_id(kind, id).await?;
        match kind {
            DatabaseKind::Postgres => self.delete_postgres(id).await?,
            DatabaseKind::Mysql => self.delete_mysql(id).await?,
            DatabaseKind::Mariadb => self.delete_mariadb(id).await?,
            DatabaseKind::Mongo => self.delete_mongo(id).await?,
            DatabaseKind::Redis => self.delete_redis(id).await?,
            DatabaseKind::Libsql => self.delete_libsql(id).await?,
        }
        Ok(())
    }
}
