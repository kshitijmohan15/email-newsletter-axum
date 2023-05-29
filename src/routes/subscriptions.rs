use axum::Form;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscription_handler(Form(user): Form<FormData>) -> String {
    format!("Hey {} of email {}", user.name, user.email)
}