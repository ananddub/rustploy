use auto_route::controller;

#[controller("/api")]
mod api_test {
    use auto_route::post;

    #[get("/hello")]
    pub async fn hello() -> &'static str {
        "Hello, world!"
    }

    #[get("/joker")]
    pub async fn joker() -> &'static str {
        "Joker is here"
    }
}
