use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::project::{CreateProjectDto, PatchProjectDto, ProjectResponseDto},
    core::middleware::validator::ValidatedJson,
    services::project::ProjectService,
};
use crate::utils::jwt::claim::Claims;

type ApiError = (StatusCode, String);

pub struct ProjectController {
    service: Arc<ProjectService>,
}

#[controller("/projects")]
impl ProjectController {
    fn new(service: Arc<ProjectService>) -> Self {
        Self { service }
    }

    #[get("/{id}")]
    async fn get(&self,claim:Claims, Path(id): Path<i64>) -> Result<Json<ProjectResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(ProjectResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/organization/{organization_id}")]
    async fn list_by_organization(
        &self,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<ProjectResponseDto>>, ApiError> {
        self.service
            .list_by_organization(organization_id)
            .await
            .map(|items| items.into_iter().map(ProjectResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        ValidatedJson(body): ValidatedJson<CreateProjectDto>,
    ) -> Result<(StatusCode, Json<ProjectResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(ProjectResponseDto::from)
            .map(|project| (StatusCode::CREATED, Json(project)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchProjectDto>,
    ) -> Result<Json<ProjectResponseDto>, ApiError> {
        self.service
            .update(id, body)
            .await
            .map(ProjectResponseDto::from)
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
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "project not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (StatusCode::NOT_FOUND, "organization not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        other => {
            tracing::error!(error = %other, "project database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
