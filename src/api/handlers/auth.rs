use auto_route::controller;
use axum::extract::Path;

struct UserController {}

#[controller("auth")]
impl UserController {
    fn new() -> Self {
        return Self {};
    }

    #[get("login")]
    async fn get_user(&self, Path(id): Path<i32>) -> String {
        format!("User {id} not found!")
    }

    #[post("signup")]
    async fn create_user(&self, Path(org_id): Path<i32>) -> String {
        format!("User created in organization {org_id}!")
    }

    #[get("whoami")]
    async fn who_am_i(&self) -> String {
        "You are authenticated!".into()
    }

    #[get("logout")]
    async fn logout(&self) -> String {
        "You have been logged out!".into()
    }
}
