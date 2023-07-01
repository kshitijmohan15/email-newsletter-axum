use email_newsletter_axum::{
    configuration::get_config,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_config().expect("Config nahi mila");
    let address = format!("127.0.0.1:{}", config.application.port);
    let listener = TcpListener::bind(address)?;
    println!("{:?}", &config.database.connection_string());

    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection).await.unwrap();
    Ok(())
}