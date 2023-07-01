use std::sync::Arc;
use tower_http::trace::TraceLayer;

use axum::{
    routing::{get, post},
    Router,
};
mod healthcheck;
mod subscriptions;

pub use healthcheck::*;
use sqlx::PgPool;
pub use subscriptions::*;

async fn hello_world() -> &'static str {
    "Hi Mama!"
}
#[derive(Clone)]
pub struct AppState {
    connection: Arc<PgPool>,
}

pub fn create_routes(connection: Arc<PgPool>) -> Router {
    let state = AppState { connection };
    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", get(hello_world))
        .route("/healthcheck", get(hc_handler))
        .route("/subscriptions", post(subscription_handler))
        .with_state(state.clone())
}
