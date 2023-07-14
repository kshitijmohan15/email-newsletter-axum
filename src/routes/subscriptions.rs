use crate::routes::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
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
// #[tracing::instrument(
//     name = "Adding a new subsscriber",
//     skip(user, state),
//     fields(
//     request_id = %Uuid::new_v4(),
//     subscriber_email = %user.email,
//     subscriber_name= %user.name
//     )
//     )]

// pub async fn subscription_handler(
//     State(state): State<AppState>,
//     user: Form<FormData>,
// ) -> impl IntoResponse {
//     let db = state.connection.as_ref();
//     let uuid_final = NewId::new_v4();

//     let response = match sqlx::query!(
//         r#"
//         INSERT INTO subscriptions (id, email, name, subscribed_at)
//         VALUES ($1, $2, $3, $4)
//         "#,
//         uuid_final as NewId,
//         user.email,
//         user.name,
//         Utc::now()
//     )
//     .execute(db)
//     .await
//     {
//         Ok(res) => (StatusCode::OK, format!("Done: {:?}", res)).into_response(),
//         Err(e) => {
//             tracing::error!("Failed to execute query: {:?}", e);
//             (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response()
//         }
//     };
//     response
// }
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, state),
    fields(
    request_id = %Uuid::new_v4(),
    subscriber_email = %form.email,
    subscriber_name= %form.name
    )
    )]
pub async fn subscribe(State(state): State<AppState>, form: Form<FormData>) -> impl IntoResponse {
    let db = state.connection.as_ref();
    match insert_subscriber(&db, &form).await {
        Ok(res) => (StatusCode::OK, format!("Done: {:?}", res)).into_response(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response()
        }
    }
}
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &Pool<Postgres>, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}
