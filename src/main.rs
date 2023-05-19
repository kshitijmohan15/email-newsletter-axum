use email_newsletter_axum::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}


