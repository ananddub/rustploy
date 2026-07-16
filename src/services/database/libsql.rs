use crate::api::dto::database::{CreateDatabaseDto, PatchDatabaseDto};
use super::{DatabaseService, DatabaseRecord, DatabaseKind};

impl DatabaseService {
    pub(super) async fn select_libsql(&self, id: i64) -> sqlx::Result<DatabaseRecord> {
        sqlx::query_as!(
            DatabaseRecord,
            r#"SELECT 'libsql' AS "kind: DatabaseKind", id AS "id!: i64", name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS "database_name?", database_user AS "database_user?", external_port,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               app_status, environment_id, server_id, created_at, updated_at
               FROM libsql_dbs WHERE id = ?"#,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub(super) async fn create_libsql(
        &self,
        input: &CreateDatabaseDto,
        app_name: &str,
        image: &str,
        db_user: &str,
        db_password: &str,
    ) -> sqlx::Result<()> {
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
            input.sqld_node.as_deref().unwrap_or("PRIMARY"),
            input.sqld_primary_url,
            input.enable_namespaces.unwrap_or(0),
            input.external_port,
            input.environment_id,
            input.server_id
        )
        .execute(self.db.as_ref())
        .await?;
        Ok(())
    }

    pub(super) async fn patch_libsql(
        &self,
        id: i64,
        input: &PatchDatabaseDto,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE libsql_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id) WHERE id = ?",
            input.name,
            input.description,
            input.docker_image,
            input.external_port,
            input.command,
            input.args,
            input.env_var,
            input.memory_reservation,
            input.memory_limit,
            input.cpu_reservation,
            input.cpu_limit,
            input.replicas,
            input.server_id,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        Ok(())
    }

    pub(super) async fn delete_libsql(&self, id: i64) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM libsql_dbs WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }

    pub(super) async fn get_libsql_server_id_and_name(&self, id: i64) -> sqlx::Result<(Option<i64>, String)> {
        let r = sqlx::query!("SELECT server_id, name FROM libsql_dbs WHERE id = ?", id)
            .fetch_one(self.db.as_ref())
            .await?;
        Ok((r.server_id, r.name))
    }
}
