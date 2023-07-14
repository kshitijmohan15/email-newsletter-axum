use email_newsletter_axum::configuration::get_config;
use email_newsletter_axum::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
// this test is framework and language proof. If tomorrow i decided that I want to ditch Rust and go back to NodeJS, I can use this same test suite to keep track of my API endpoints, we would just have to change the way we spawn the app, example: change the bash script that must run before the tests begin.
use rand;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::PgPool;
use std::net::TcpListener;
// use tracing_subscriber::fmt::format;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);
});

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/healthcheck", app.address))
        .send()
        .await
        .expect("Failed to execute request");
    // checking if the health check always returns a 200
    assert!(response.status().is_success());
    // making sure that the health check’s response has no body
    assert_eq!(Some(13), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let n1: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let body = format!("name=le%20guin&email=ursula_le_guin%{}gmail.com", n1);
    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
}

// desrialization errors lead to 422 errors in axum (https://github.com/tokio-rs/axum/issues/1680)
#[tokio::test]
async fn subscribe_returns_422_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 422 Bad Request when the payload was {}.",
            error_message
        )
    }
}
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
// The function is asynchronous now!
async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let configuration = get_config().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let server = email_newsletter_axum::startup::run(listener, connection_pool.clone());
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}
