use std::net::SocketAddr;
mod routes;
use routes::create_routes;

pub async fn run() -> Result<(), std::io::Error> {
    let app = create_routes();
    tracing_subscriber::fmt::init();
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    log::info!("Listening on port http://{}", socket_addr);
    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service()).await.unwrap();
    Ok(server)
}
