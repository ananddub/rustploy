use axum::Error;
use axum::extract::Query;
use git2::{Direction, Repository};
use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use auto_route::{controller, get};


pub struct Public;
#[controller("/public")]
impl Public {
    pub fn new() -> Self {
        Self
    }
    #[get("/git")]
    pub async fn get_git(&self,Query(query) : Query<String>)->Result<Vec<String>,Error> {
        let temp = TempDir::new().map_err(|e| axum::Error::from(e.into()))?;
        let repo = Repository::init_bare(temp.path()).map_err(|e| axum::Error::from(e.into()))?;
        let mut remote = repo.remote_anonymous(&query).map_err(|e| axum::Error::from(e.into()))?;
        remote.connect(Direction::Fetch).map_err(|err| axum::Error::from(err))?;
        let mut branches = Vec::new();
        for head in remote.list().map_err(|err| axum::Error::from(err))? {
            branches.push(head.name().to_string());
        }

        remote.disconnect()?;
        Ok(branches)
    }

}
