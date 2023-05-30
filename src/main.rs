use email_newsletter_axum::{configuration::get_config, startup::run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.unwrap().application.port))
        .expect("Bind nahi ho paaya");
    let finally = run(listener).await;
    finally
    // only readx yaml file as configuration
}
