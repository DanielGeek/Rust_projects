mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;

use axum::{
    body::Body,
    routing::{get, patch, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_user_agent::mirror_user_agent;
use path_variables::{path_variables, hard_coded_path};
use query_params::query_params;
use mirror_custom_header::mirror_custom_header;

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", patch(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
}
