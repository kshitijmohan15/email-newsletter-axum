use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use serde::Deserialize;
use std::ops::Deref;

use crate::routes::AppState;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscription_handler(
    State(state): State<AppState>,
    user: Option<Form<FormData>>,
) -> impl IntoResponse {
    let Form(form) = user.unwrap();
    let db_ref = state.connection.as_ref();
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_ref)
    .await
    {
        Ok(_) => (StatusCode::ACCEPTED, format!("Done: ")),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            (StatusCode::NOT_FOUND, format!("Not Found: "))
        }
    };
    println!("{:?}", db_ref.deref());
    format!("Hey {} of email {}", form.name, form.email)
}
