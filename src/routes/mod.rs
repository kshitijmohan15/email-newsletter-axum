use hyper::{Body, Request};
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
        .route("/", get(hello_world))
        .route("/healthcheck", get(hc_handler))
        .route("/subscriptions", post(subscribe))
        .route("/subscriptions", get(get_all_subscribers))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<Body>| {
                    tracing::debug_span!("http-request", status_code = tracing::field::Empty,)
                })
        )
        .with_state(state)
}
