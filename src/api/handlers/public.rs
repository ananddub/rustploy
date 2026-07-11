use auto_route::controller;
use axum::Json;
use axum::extract::Query;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::utils::git::GitCli;

#[derive(Deserialize, poem_openapi::Object)]
pub struct BranchQuery {
    pub query: String,
}

pub struct Public;
#[controller("/public")]
impl Public {
    pub fn new() -> Self {
        Self
    }

    #[get("/git/list_branches")]
    pub async fn list_branches(
        &self,
        Query(params): Query<BranchQuery>,
    ) -> Result<Json<Vec<String>>, StatusCode> {
        let repository_url = params.query.trim();
        if repository_url.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }

        GitCli::new_local()
            .remote_branches(repository_url)
            .await
            .map(|branches| Json(branches.into_iter().map(|branch| branch.name).collect()))
            .map_err(|error| {
                tracing::warn!(
                    repository_url,
                    error = %error,
                    "could not list git remote branches"
                );
                StatusCode::BAD_REQUEST
            })
    }
}
