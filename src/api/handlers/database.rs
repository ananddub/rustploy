use std::{str::FromStr, sync::Arc};

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

type ApiError = (StatusCode, String);

pub struct DatabaseController {
    service: Arc<DatabaseService>,
}

#[controller("/databases")]
impl DatabaseController {
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
            .map(|items| items.into_iter().map(DatabaseResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{kind}/{id}")]
    async fn get(
        &self,
        _claims: Claims,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<Json<DatabaseResponseDto>, ApiError> {
        let kind = parse_kind(kind)?;
        self.service
            .get_by_id(kind, id)
            .await
            .map(DatabaseResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/{kind}")]
    async fn create(
        &self,
        _claims: Claims,
        Path(kind): Path<String>,
        ValidatedJson(body): ValidatedJson<CreateDatabaseDto>,
    ) -> Result<(StatusCode, Json<DatabaseResponseDto>), ApiError> {
        let kind = parse_kind(kind)?;
        self.service
            .create(kind, body)
            .await
            .map(DatabaseResponseDto::from)
            .map(|database| (StatusCode::CREATED, Json(database)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{kind}/{id}")]
    async fn patch(
        &self,
        _claims: Claims,
        Path((kind, id)): Path<(String, i64)>,
        ValidatedJson(body): ValidatedJson<PatchDatabaseDto>,
    ) -> Result<Json<DatabaseResponseDto>, ApiError> {
        let kind = parse_kind(kind)?;
        self.service
            .patch(kind, id, body)
            .await
            .map(DatabaseResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/{kind}/{id}/deploy")]
    async fn deploy(
        &self,
        _claims: Claims,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        self.operation(kind, id, DatabaseOperation::Deploy).await
    }

    #[post("/{kind}/{id}/redeploy")]
    async fn redeploy(
        &self,
        _claims: Claims,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        self.operation(kind, id, DatabaseOperation::Redeploy).await
    }

    #[post("/{kind}/{id}/reload")]
    async fn reload(
        &self,
        _claims: Claims,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        self.operation(kind, id, DatabaseOperation::Reload).await
    }

    #[post("/{kind}/{id}/start")]
    async fn start(
        &self,
        _claims: Claims,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        self.operation(kind, id, DatabaseOperation::Start).await
    }

    #[post("/{kind}/{id}/stop")]
    async fn stop(
        &self,
        _claims: Claims,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        self.operation(kind, id, DatabaseOperation::Stop).await
    }

    #[delete("/{kind}/{id}")]
    async fn delete(
        &self,
        _claims: Claims,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<StatusCode, ApiError> {
        let kind = parse_kind(kind)?;
        self.service
            .delete(kind, id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }

    async fn operation(
        &self,
        kind: String,
        id: i64,
        operation: DatabaseOperation,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        let kind = parse_kind(kind)?;
        self.service
            .run_operation(kind, id, operation)
            .await
            .map(DatabaseOperationResponseDto::from)
            .map(|response| (StatusCode::ACCEPTED, Json(response)))
            .map_err(map_sqlx_error)
    }
}

fn parse_kind(kind: String) -> Result<DatabaseKind, ApiError> {
    DatabaseKind::from_str(&kind).map_err(|()| {
        (
            StatusCode::BAD_REQUEST,
            "database kind must be postgres, mysql, mariadb, mongo, redis, or libsql".into(),
        )
    })
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "database not found".into()),
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
            tracing::error!(error = %other, "managed database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
