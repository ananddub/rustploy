use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::organization::{
        CreateOrganizationDto, OrganizationResponseDto, PatchOrganizationDto,
    },
    core::middleware::validator::ValidatedJson,
    services::organization::OrganizationService,
};

type ApiError = (StatusCode, String);

pub struct OrganizationController {
    service: Arc<OrganizationService>,
}

#[controller("/organizations")]
impl OrganizationController {
    fn new(service: Arc<OrganizationService>) -> Self {
        Self { service }
    }

    #[get("/{id}")]
    async fn get(&self, Path(id): Path<i64>) -> Result<Json<OrganizationResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(OrganizationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/owner/{owner_id}")]
    async fn list_by_owner(
        &self,
        Path(owner_id): Path<i64>,
    ) -> Result<Json<Vec<OrganizationResponseDto>>, ApiError> {
        self.service
            .list_by_owner(owner_id)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(OrganizationResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        ValidatedJson(body): ValidatedJson<CreateOrganizationDto>,
    ) -> Result<(StatusCode, Json<OrganizationResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(OrganizationResponseDto::from)
            .map(|organization| (StatusCode::CREATED, Json(organization)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchOrganizationDto>,
    ) -> Result<Json<OrganizationResponseDto>, ApiError> {
        self.service
            .update(id, body)
            .await
            .map(OrganizationResponseDto::from)
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
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "organization not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (StatusCode::NOT_FOUND, "owner not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error = %other, "organization database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
