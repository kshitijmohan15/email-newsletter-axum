// this test is framework and language proof. If tomorrow i decided that I want to ditch Rust and go back to NodeJS, I can use this same test suite to keep track of my API endpoints, we would just have to change the way we spawn the app, example: change the bash script that must run before the tests begin.
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/healthcheck", addr))
        .send()
        .await
        .expect("Failed to execute request");
    // checking if the health check always returns a 200
    assert!(response.status().is_success());
    // making sure that the health check’s response has no body
    assert_eq!(Some(13), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app_addr = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    println!("Bhai {}", format!("{}/subscriptions", &app_addr));
    let response = client
        .post(&format!("{}/subscriptions", &app_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");
    // print!("Error hai {:?}", response.error_for_status());
    assert_eq!(200, response.status().as_u16())
}

// desrialization errors lead to 422 errors in axum (https://github.com/tokio-rs/axum/issues/1680)
#[tokio::test]
async fn subscribe_returns_422_when_data_is_missing() {
    let app_addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_addr))
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

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter_axum::startup::run(listener);
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
