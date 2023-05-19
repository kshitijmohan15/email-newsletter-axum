use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn hc_handler() -> &'static str {
    "Hey"
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/healthcheck", get(hc_handler))
}
