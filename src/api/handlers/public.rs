use auto_route::{controller, get};
use axum::Json;
use axum::extract::Query;
// use git2::{Direction, Repository};
use reqwest::StatusCode;
use serde::Deserialize;
use tempfile::TempDir;
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
    ) -> Result<Json<Vec<&str>>, StatusCode> {

        let mut git = GitCli::new_local();
        git.set_repository(params.query.clone());
        let  remote = git.branches().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        let mut branches:Vec<&str> = Vec::new();

        for head in remote.iter() {
            branches.push(&head.name);
        }


        Ok(Json(branches))
    }
}
