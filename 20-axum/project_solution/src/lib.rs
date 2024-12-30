use std::net::SocketAddr;

use router::create_router;
use sea_orm::Database;
use utilities::app_state::AppState;

mod database;
mod router;
mod routes;
mod utilities;


pub async fn run(database_url: String) {
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    let app_state = AppState { db };
    let app = create_router(app_state);
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
