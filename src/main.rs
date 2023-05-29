use email_newsletter_axum::{configuration::get_config, startup::run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // let config = build_configuration().expect("Not looking good bruv");
    // let configs = Res{settings: config.try_into()?};
    let config = get_config().expect("Could not fetch config");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.settings.application_port))
        .expect("Bind nahi ho paaya");
    let finally = run(listener).await;
    finally
}
