use email_newsletter_axum::run;
use std::net::{TcpListener};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    run(listener).await
}
