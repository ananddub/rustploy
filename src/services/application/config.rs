use crate::api::dto::application::{PatchBuildConfigDto, PatchResourceConfigDto};

use super::{ApplicationRecord, ApplicationService};

impl ApplicationService {
    pub async fn patch_build_config(
        &self,
        id: i64,
        input: PatchBuildConfigDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               build_args = COALESCE(?, build_args), build_secrets = COALESCE(?, build_secrets),
               dockerfile = COALESCE(?, dockerfile), docker_context_path = COALESCE(?, docker_context_path),
               docker_build_stage = COALESCE(?, docker_build_stage), publish_directory = COALESCE(?, publish_directory),
               is_static_spa = COALESCE(?, is_static_spa), create_env_file = COALESCE(?, create_env_file),
               railpack_version = COALESCE(?, railpack_version), heroku_version = COALESCE(?, heroku_version),
               command = COALESCE(?, command), args = COALESCE(?, args), build_path = COALESCE(?, build_path),
               clean_cache = COALESCE(?, clean_cache), enable_submodules = COALESCE(?, enable_submodules),
               watch_paths = COALESCE(?, watch_paths)
               WHERE id = ?"#,
            input.build_args,
            input.build_secrets,
            input.dockerfile,
            input.docker_context_path,
            input.docker_build_stage,
            input.publish_directory,
            input.is_static_spa,
            input.create_env_file,
            input.railpack_version,
            input.heroku_version,
            input.command,
            input.args,
            input.build_path,
            input.clean_cache,
            input.enable_submodules,
            input.watch_paths,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }

    pub async fn patch_resource_config(
        &self,
        id: i64,
        input: PatchResourceConfigDto,
    ) -> sqlx::Result<ApplicationRecord> {
        sqlx::query!(
            r#"UPDATE applications SET
               memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit),
               cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit),
               replicas = COALESCE(?, replicas)
               WHERE id = ?"#,
            input.memory_reservation,
            input.memory_limit,
            input.cpu_reservation,
            input.cpu_limit,
            input.replicas,
            id
        )
        .execute(self.db.as_ref())
        .await?;
        self.get_by_id(id).await
    }
}
