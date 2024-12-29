mod router;
mod routes;

use router::create_router;

use std::net::SocketAddr;

pub async fn run() {
    let app = create_router();

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
