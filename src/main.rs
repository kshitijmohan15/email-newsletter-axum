use email_newsletter_axum::run;
use std::net::{SocketAddr, TcpListener};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = TcpListener::bind("127.0.0.1:0").unwrap();
    run(port).await
}
