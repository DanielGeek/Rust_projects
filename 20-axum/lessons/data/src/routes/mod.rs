mod create_task;
mod custom_json_extractor;
mod delete_task;
mod get_tasks;
mod guard;
mod hello_world;
mod partial_update_tasks;
mod update_tasks;
mod users;
mod validate_with_serde;

use axum::{
    extract::FromRef,
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use delete_task::delete_task;
use get_tasks::{get_all_tasks, get_one_task};
use guard::guard;
use partial_update_tasks::partial_update;
use sea_orm::DatabaseConnection;
use update_tasks::atomic_update;
use users::{create_user, login, logout};
use validate_with_serde::validate_with_serde;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    Router::new()
        .route("/users/logout", post(logout))
        .route("/hello_world", get(hello_world::hello_world))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        // put other routes above route_layer to protect them
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/{task_id}", get(get_one_task))
        .route("/tasks/{task_id}", put(atomic_update))
        .route("/tasks/{task_id}", patch(partial_update))
        .route("/tasks/{task_id}", delete(delete_task))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .with_state(app_state)
}
