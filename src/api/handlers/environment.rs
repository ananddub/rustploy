use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::environment::{CreateEnvironmentDto, EnvironmentResponseDto, PatchEnvironmentDto},
    core::middleware::validator::ValidatedJson,
    services::environment::EnvironmentService,
};

type ApiError = (StatusCode, String);

pub struct EnvironmentController {
    service: Arc<EnvironmentService>,
}

#[controller("/environments")]
impl EnvironmentController {
    fn new(service: Arc<EnvironmentService>) -> Self {
        Self { service }
    }

    #[get("/{id}")]
    async fn get(&self, Path(id): Path<i64>) -> Result<Json<EnvironmentResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(EnvironmentResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/project/{project_id}")]
    async fn list_by_project(
        &self,
        Path(project_id): Path<i64>,
    ) -> Result<Json<Vec<EnvironmentResponseDto>>, ApiError> {
        self.service
            .list_by_project(project_id)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(EnvironmentResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        ValidatedJson(body): ValidatedJson<CreateEnvironmentDto>,
    ) -> Result<(StatusCode, Json<EnvironmentResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(EnvironmentResponseDto::from)
            .map(|environment| (StatusCode::CREATED, Json(environment)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchEnvironmentDto>,
    ) -> Result<Json<EnvironmentResponseDto>, ApiError> {
        self.service
            .update(id, body)
            .await
            .map(EnvironmentResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[put("/{id}/default")]
    async fn set_default(
        &self,
        Path(id): Path<i64>,
    ) -> Result<Json<EnvironmentResponseDto>, ApiError> {
        self.service
            .set_default(id)
            .await
            .map(EnvironmentResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(&self, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "environment not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (StatusCode::NOT_FOUND, "project not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error = %other, "environment database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
