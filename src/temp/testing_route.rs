use auto_route::controller;

#[controller("/api")]
mod api_test {

    #[get("/hello")]
    pub async fn hello() -> &'static str {
        "Hello, world!"
    }
}
