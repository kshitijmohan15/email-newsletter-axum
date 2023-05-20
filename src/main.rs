use std::net::SocketAddr;
use email_newsletter_axum::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port: SocketAddr = "127.0.0.1:3000".parse().unwrap(); 
    run(port).await
}


