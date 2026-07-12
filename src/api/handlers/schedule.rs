use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::schedule::{
        CreateScheduleDto, PatchScheduleDto, ScheduleResponseDto, ScheduleRunResponseDto,
    },
    core::middleware::validator::ValidatedJson,
    services::schedule::ScheduleService,
    utils::jwt::claim::Claims,
};

type ApiError = (StatusCode, String);

pub struct ScheduleController {
    service: Arc<ScheduleService>,
}

#[controller("/schedules")]
impl ScheduleController {
    fn new(service: Arc<ScheduleService>) -> Self {
        Self { service }
    }

    #[get("/{id}")]
    async fn get(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<ScheduleResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(ScheduleResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/application/{application_id}")]
    async fn list_by_application(
        &self,
        _claims: Claims,
        Path(application_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        self.service
            .list_by_application(application_id)
            .await
            .map(|items| items.into_iter().map(ScheduleResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/compose/{compose_id}")]
    async fn list_by_compose(
        &self,
        _claims: Claims,
        Path(compose_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        self.service
            .list_by_compose(compose_id)
            .await
            .map(|items| items.into_iter().map(ScheduleResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/server/{server_id}")]
    async fn list_by_server(
        &self,
        _claims: Claims,
        Path(server_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        self.service
            .list_by_server(server_id)
            .await
            .map(|items| items.into_iter().map(ScheduleResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/organization/{organization_id}")]
    async fn list_by_organization(
        &self,
        _claims: Claims,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        self.service
            .list_by_organization(organization_id)
            .await
            .map(|items| items.into_iter().map(ScheduleResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        _claims: Claims,
        ValidatedJson(body): ValidatedJson<CreateScheduleDto>,
    ) -> Result<(StatusCode, Json<ScheduleResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(ScheduleResponseDto::from)
            .map(|schedule| (StatusCode::CREATED, Json(schedule)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchScheduleDto>,
    ) -> Result<Json<ScheduleResponseDto>, ApiError> {
        self.service
            .patch(id, body)
            .await
            .map(ScheduleResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/enable")]
    async fn enable(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<ScheduleResponseDto>, ApiError> {
        self.service
            .set_enabled(id, true)
            .await
            .map(ScheduleResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/disable")]
    async fn disable(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<ScheduleResponseDto>, ApiError> {
        self.service
            .set_enabled(id, false)
            .await
            .map(ScheduleResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/run")]
    async fn run_now(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ScheduleRunResponseDto>), ApiError> {
        self.service
            .run_now(id)
            .await
            .map(ScheduleRunResponseDto::from)
            .map(|response| (StatusCode::ACCEPTED, Json(response)))
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
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "schedule not found".into()),
        sqlx::Error::Protocol(message) if message.contains("already running") => {
            (StatusCode::CONFLICT, message)
        }
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
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
            tracing::error!(error = %other, "schedule database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
