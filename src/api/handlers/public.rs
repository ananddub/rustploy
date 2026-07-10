use auto_route::{controller, get};
use axum::Json;
use axum::extract::Query;
use git2::{Direction, Repository};
use reqwest::StatusCode;
use serde::Deserialize;
use tempfile::TempDir;

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
        let temp = TempDir::new().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let repo =
            Repository::init_bare(temp.path()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut remote = repo
            .remote_anonymous(&params.query)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        remote
            .connect(Direction::Fetch)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let mut branches = Vec::new();

        for head in remote.list().map_err(|_| StatusCode::BAD_REQUEST)? {
            branches.push(head.name().to_string());
        }

        remote
            .disconnect()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(branches))
    }
}
