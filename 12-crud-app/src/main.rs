use axum::Router;
use axum::routing::{delete, get, post, put};
use controller::{create_user, get_user_by_id, list_users, update_user, delete_user};

mod model;
mod controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(list_users))
        .route("/user/:id", get(get_user_by_id))
        .route("/user", post(create_user))
        .route("/user/:id", put(update_user))
        .route("/user/:id", delete(delete_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
