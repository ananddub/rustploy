use std::sync::Arc;

use auto_route::controller;
use axum::{extract::Path, http::StatusCode};

use crate::{
    services::deployment::{CancelDeploymentResult, DeploymentService},
    utils::jwt::claim::Claims,
};

type ApiError = (StatusCode, String);

pub struct DeploymentController {
    service: Arc<DeploymentService>,
}

#[controller("/deployments")]
impl DeploymentController {
    fn new(service: Arc<DeploymentService>) -> Self {
        Self { service }
    }

    #[post("/{id}/cancel")]
    async fn cancel(&self, _claims: Claims, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        match self.service.cancel(id).await {
            Ok(CancelDeploymentResult::CancelRequested) => Ok(StatusCode::ACCEPTED),
            Ok(CancelDeploymentResult::NotRunning) => Err((
                StatusCode::CONFLICT,
                "deployment is not running, so it cannot be cancelled".into(),
            )),
            Ok(CancelDeploymentResult::NotCancellable) => Err((
                StatusCode::BAD_REQUEST,
                "deployment is not attached to an application or compose project".into(),
            )),
            Ok(CancelDeploymentResult::NotActiveInThisProcess) => Err((
                StatusCode::CONFLICT,
                "deployment is not active in this process; it may already be finished or recovered after restart".into(),
            )),
            Err(error) => Err(map_sqlx_error(error)),
        }
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "deployment not found".into()),
        other => {
            tracing::error!(error = %other, "deployment operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "deployment operation failed".into(),
            )
        }
    }
}
