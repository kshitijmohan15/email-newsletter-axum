use crate::routes::AppState;
use ::uuid::Uuid as NewId;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct GetFormData {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}
pub async fn get_all_subscribers(State(state): State<AppState>) -> impl IntoResponse {
    let db = state.connection.as_ref();

    let query_result = sqlx::query_as!(GetFormData, "SELECT id, email, name FROM subscriptions")
        .fetch_all(db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let subscribers = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": subscribers.len(),
        "subscribers": subscribers
    });
    Ok(Json(json_response))
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
