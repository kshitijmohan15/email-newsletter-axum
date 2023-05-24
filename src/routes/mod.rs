use axum::{
    routing::{get, post},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};

async fn hello_world() -> &'static str {
    "Hi Mama!"
}

async fn hc_handler() -> &'static str {
    "Hello, World!"
}
#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}
#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    service: String,
}

async fn subscription_handler(Form(user): Form<FormData>) -> String {
    format!("Hey {} of email {}", user.name, user.email)
}
async fn users_handler() -> Json<Vec<User>> {
    // create and return a JSON response with 100 users
    let mut users = vec![];
    for i in 0..100 {
        users.push(User {
            name: format!("User {}", i),
            service: format!("Service {}", i),
        })
    }
    Json(users)
}
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/healthcheck", get(hc_handler))
        .route("/subscriptions", post(subscription_handler))
        .route("/users", get(users_handler))
}
