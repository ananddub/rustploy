use axum::{
    extract::{FromRequest, Json, Request},
    http::StatusCode,
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    Json<T>: FromRequest<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON".into()))?;

        value
            .validate()
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        Ok(ValidatedJson(value))
    }
}
