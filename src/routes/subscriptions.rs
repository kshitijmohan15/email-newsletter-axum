use crate::routes::AppState;
use ::uuid::Uuid as NewId;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

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
    let _request_id = uuid::Uuid::new_v4();

    let _request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
    "Adding a new subscriber.",
    %_request_id,
    subscriber_email = %user.email,
    subscriber_name= %user.name
    );
    let _request_span_guard = request_span.enter();

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
        Ok(res) => (StatusCode::OK, format!("Done: {:?}", res)).into_response(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            (StatusCode::FORBIDDEN, format!("{}", e)).into_response()
        }
    };
    response
}


