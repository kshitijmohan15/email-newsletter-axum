use std::net::SocketAddr;
mod routes;
use routes::create_routes;

pub async fn run(addr: SocketAddr) -> Result<(), std::io::Error> {
    let app = create_routes();
    tracing_subscriber::fmt::init();
    log::info!("Listening on port http://{}", &addr);
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(server)
}
