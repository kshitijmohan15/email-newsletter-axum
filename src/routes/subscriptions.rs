use crate::routes::AppState;
use ::uuid::Uuid as NewId;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscription_handler(
    State(state): State<AppState>,
    user: Form<FormData>,
) -> impl IntoResponse {
    let db = state.connection.as_ref();
    let uuid_final = NewId::new_v4();
    let response = match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid_final as NewId,
        user.email,
        user.name,
        Utc::now()
    )
    .execute(db)
    .await
    {
        Ok(_) => (StatusCode::ACCEPTED, format!("Done: ")).into_response(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            (StatusCode::FORBIDDEN, format!("Not Found: ")).into_response()
        }
    };
    response
}
