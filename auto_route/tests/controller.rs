use auto_di::Container;
use auto_route::{build_routes, controller, get};
use axum::{
    Json,
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

struct UserController {
    prefix: String,
}

#[get("/health")]
async fn health() -> &'static str {
    "ok"
}

#[controller("/module")]
mod module_routes {
    use axum::extract::Path;

    #[get("/:id")]
    async fn find(Path(id): Path<u64>) -> String {
        format!("module-{id}")
    }
}

#[controller("/users")]
impl UserController {
    pub fn new() -> Self {
        Self {
            prefix: "user".to_owned(),
        }
    }

    #[get("/:id")]
    async fn get_user(&self, Path(id): Path<u64>) -> Json<Value> {
        Json(json!({ "name": format!("{}-{id}", self.prefix) }))
    }

    #[post]
    async fn create_user(&self, Json(body): Json<Value>) -> (StatusCode, Json<Value>) {
        (StatusCode::CREATED, Json(body))
    }
}

#[tokio::test]
async fn dispatches_get_and_post_to_the_di_managed_controller() {
    let container = Container::new().unwrap();
    let app = build_routes(&container).await.unwrap();

    let response = app
        .clone()
        .oneshot(Request::get("/users/42").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(
        serde_json::from_slice::<Value>(&body).unwrap(),
        json!({ "name": "user-42" })
    );

    let response = app
        .clone()
        .oneshot(
            Request::post("/users")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Ada"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let response = app
        .clone()
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"ok");

    let response = app
        .oneshot(Request::get("/module/7").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"module-7");
}
