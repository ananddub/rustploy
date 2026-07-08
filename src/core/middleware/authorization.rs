
use auto_di::resolve;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
    Json,
};
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use serde_json::json;
use crate::utils::jwt::claim::Claims;
use crate::utils::jwt::error::TokenError;
use crate::utils::jwt::service::JwtService;

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, Json(json!({ "error": "missing header" }))))?;
        let jwt_service = resolve::<JwtService>().await.map_err(|_| (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "jwt service unavailable" })),
                ))?
            .clone();
        jwt_service
            .validate_access_token(bearer.token())
            .map_err(|_| (StatusCode::UNAUTHORIZED, Json(json!({ "error": "invalid token" }))))
    }
}