use crate::routes::create_routes;
use sqlx::PgPool;
use std::{net::TcpListener, sync::Arc};

pub async fn run(listener: TcpListener, connection: PgPool) -> Result<(), std::io::Error> {
    let connection = Arc::new(connection);
    let app = create_routes(connection);
    println!(
        "👂 Listening on port http://{:?}",
        listener.local_addr().unwrap()
    );
    //     Each worker runs its own copy of the application built by Server calling the very same app.into_make_service() that axum::Server.serve() takes as argument.
    // That is why connection has to be cloneable - we need to have one for every copy of App.
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
