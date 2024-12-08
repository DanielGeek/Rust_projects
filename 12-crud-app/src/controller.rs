use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::model::{User, UserInfo};

pub async fn list_users() -> (StatusCode, Json<Value>) {
    // get users
}

pub async fn get_user_by_id(Path(id): Path<u64>) -> (StatusCode, Json<Value>) {
    // get user by id
}

pub async fn create_user(Json(user): Json<UserInfo>) -> StatusCode {
    // create user
}

pub async fn update_user(Path(id): Path<u64>, Json(user): Json<UserInfo>) -> StatusCode {
    // update user
}

pub async fn delete_user(Path(id): Path<u64>) -> StatusCode {
    // delete user
}
