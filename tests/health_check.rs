// this test is framework and language proof. If tomorrow i decided that I want to ditch Rust and go back to NodeJS, I can use this same test suite to keep track of my API endpoints, we would just have to change the way we spawn the app, example: change the bash script that must run before the tests begin.

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:3000/healthcheck")
        .send()
        .await
        .expect("Failed to execute request");
    // checking if the health check always returns a 200
    assert!(response.status().is_success());
    // making sure that the health check’s response has no body
    assert_eq!(Some(3), response.content_length());
}

fn spawn_app()  {
    let server = email_newsletter_axum::run("127.0.0.1:3000".parse().unwrap());
    let _ = tokio::spawn(server);
}
