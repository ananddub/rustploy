use auto_di::singleton;
use auto_route::controller;

pub struct OrganizationController;

#[singleton]
#[controller("organization")]
impl OrganizationController {
    fn new() -> Self {
        Self
    }

    #[get("/list")]
    async fn list(&self) -> String {
        "List of organizations".to_string()
    }
}
