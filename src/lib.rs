use std::net::TcpListener;
mod routes;
use routes::create_routes;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let app = create_routes();
    println!(
        "Listening on port http://{:?}",
        listener.local_addr().unwrap()
    );
    let server = axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(server)
}
