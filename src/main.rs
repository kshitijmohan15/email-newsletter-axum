use email_newsletter_axum::{configuration::get_config, startup::run};
use sqlx::{PgConnection, Connection, PgPool};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Config nahi mila");
    let address = format!("127.0.0.1:{}", config.application.port);
    let listener = TcpListener::bind(address)?;
    println!("{:?}", &config.database.connection_string());
    let connection = PgPool::connect(&config.database.connection_string())
    .await
    .expect("Failed to connect to Postgres.");

    let finally = run(listener, connection).await;
    finally
}
