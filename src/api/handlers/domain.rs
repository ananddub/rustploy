use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::domain::{CreateDomainDto, DomainResponseDto, PatchDomainDto},
    core::middleware::validator::ValidatedJson,
    services::domain::DomainService,
    utils::jwt::claim::Claims,
};

type ApiError = (StatusCode, String);

pub struct DomainController {
    service: Arc<DomainService>,
}

#[controller("/domains")]
impl DomainController {
    fn new(service: Arc<DomainService>) -> Self {
        Self { service }
    }

    #[get("/{id}")]
    async fn get(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<DomainResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(DomainResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/application/{application_id}")]
    async fn list_by_application(
        &self,
        _claims: Claims,
        Path(application_id): Path<i64>,
    ) -> Result<Json<Vec<DomainResponseDto>>, ApiError> {
        self.service
            .list_by_application(application_id)
            .await
            .map(|items| items.into_iter().map(DomainResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/compose/{compose_id}")]
    async fn list_by_compose(
        &self,
        _claims: Claims,
        Path(compose_id): Path<i64>,
    ) -> Result<Json<Vec<DomainResponseDto>>, ApiError> {
        self.service
            .list_by_compose(compose_id)
            .await
            .map(|items| items.into_iter().map(DomainResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        _claims: Claims,
        ValidatedJson(body): ValidatedJson<CreateDomainDto>,
    ) -> Result<(StatusCode, Json<DomainResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(DomainResponseDto::from)
            .map(|domain| (StatusCode::CREATED, Json(domain)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchDomainDto>,
    ) -> Result<Json<DomainResponseDto>, ApiError> {
        self.service
            .patch(id, body)
            .await
            .map(DomainResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(&self, _claims: Claims, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "domain not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (StatusCode::NOT_FOUND, "related resource not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_check_violation() => {
            (StatusCode::BAD_REQUEST, database_error.message().into())
        }
        other => {
            tracing::error!(error = %other, "domain database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "domain operation failed".into(),
            )
        }
    }
}
