use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::database::{
        CreateDatabaseDto, DatabaseOperationResponseDto, DatabaseResponseDto, PatchDatabaseDto,
    },
    core::middleware::validator::ValidatedJson,
    services::database::{DatabaseKind, DatabaseOperation, DatabaseService},
    utils::jwt::claim::Claims,
};
use super::ApiError;

pub struct RedisController {
    service: Arc<DatabaseService>,
}

#[controller("/redis")]
impl RedisController {
    fn new(service: Arc<DatabaseService>) -> Self {
        Self { service }
    }

    #[get("/environment/{environment_id}")]
    async fn list_by_environment(
        &self,
        _claims: Claims,
        Path(environment_id): Path<i64>,
    ) -> Result<Json<Vec<DatabaseResponseDto>>, ApiError> {
        self.service
            .list_by_environment(environment_id)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .filter(|item| item.kind == "redis")
                    .map(DatabaseResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<DatabaseResponseDto>, ApiError> {
        self.service
            .get_by_id(DatabaseKind::Redis, id)
            .await
            .map(DatabaseResponseDto::from)
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        _claims: Claims,
        ValidatedJson(body): ValidatedJson<CreateDatabaseDto>,
    ) -> Result<(StatusCode, Json<DatabaseResponseDto>), ApiError> {
        self.service
            .create(DatabaseKind::Redis, body)
            .await
            .map(DatabaseResponseDto::from)
            .map(|database| (StatusCode::CREATED, Json(database)))
            .map_err(super::map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchDatabaseDto>,
    ) -> Result<Json<DatabaseResponseDto>, ApiError> {
        self.service
            .patch(DatabaseKind::Redis, id, body)
            .await
            .map(DatabaseResponseDto::from)
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[post("/{id}/deploy")]
    async fn deploy(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(&self.service, DatabaseKind::Redis, id, DatabaseOperation::Deploy).await
    }

    #[post("/{id}/redeploy")]
    async fn redeploy(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(&self.service, DatabaseKind::Redis, id, DatabaseOperation::Redeploy).await
    }

    #[post("/{id}/reload")]
    async fn reload(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(&self.service, DatabaseKind::Redis, id, DatabaseOperation::Reload).await
    }

    #[post("/{id}/start")]
    async fn start(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(&self.service, DatabaseKind::Redis, id, DatabaseOperation::Start).await
    }

    #[post("/{id}/stop")]
    async fn stop(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(&self.service, DatabaseKind::Redis, id, DatabaseOperation::Stop).await
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(DatabaseKind::Redis, id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(super::map_sqlx_error)
    }
}
