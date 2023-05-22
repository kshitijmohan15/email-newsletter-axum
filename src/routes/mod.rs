use axum::{
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn hc_handler() -> &'static str {
    "Hi Papa"
}
#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}
async fn subscription_handler(Form(user): Form<FormData>) -> String {
    format!("Hey {} of email {}", user.name, user.email)
}
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/healthcheck", get(hc_handler))
        .route("/subscriptions", post(subscription_handler))
}
