use crate::routes::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use chrono::Utc;
use::uuid::Uuid as NewId;
use serde::Deserialize;
use std::ops::Deref;

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
    let db = state.connection.as_ref();
    let uuid_final = NewId::new_v4();
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid_final as NewId,
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db)
    .await
    {
        Ok(_) => (StatusCode::ACCEPTED, format!("Done: ")),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            (StatusCode::NOT_FOUND, format!("Not Found: "))
        }
    };
    println!("{:?}", db.deref());
    format!("Hey {} of email {}", form.name, form.email)
}
