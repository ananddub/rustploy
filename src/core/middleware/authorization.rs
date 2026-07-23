use crate::services::auth::AuthService;
use crate::utils::jwt::claim::Claims;
use auto_di::resolve;
use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use serde_json::json;

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = if let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            bearer.token().to_string()
        } else {
            let query = parts.uri.query().unwrap_or_default();
            query
                .split('&')
                .find_map(|pair| {
                    let mut kv = pair.splitn(2, '=');
                    if kv.next()? == "token" {
                        Some(kv.next()?.to_string())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "error": "missing authorization header or query token" })),
                    )
                })?
        };

        let auth_service = resolve::<AuthService>()
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "auth service unavailable" })),
                )
            })?
            .clone();

        auth_service
            .validate_access_token(&token)
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "invalid token" })),
                )
            })
    }
}
