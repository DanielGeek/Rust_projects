use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;

async fn index_handler() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening...!");

    axum::serve(listener, app).await.unwrap();
}
