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
    assert_eq!(Some(3), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter_axum::run(listener);
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
