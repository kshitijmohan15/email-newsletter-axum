use axum::{
    routing::{get, post},
    Router,
};
mod healthcheck;
mod subscriptions;

pub use healthcheck::*;
pub use subscriptions::*;

async fn hello_world() -> &'static str {
    "Hi Mama!"
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/healthcheck", get(hc_handler))
        .route("/subscriptions", post(subscription_handler))
}
