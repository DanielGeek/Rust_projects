use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use crate::database::users::Entity as Users;

use crate::database::users;

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

pub async fn create_user(
    Json(request_user): Json<RequestUser>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some("n12121212jasjajsas".to_owned())),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    Json(request_user): Json<RequestUser>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        let new_token = "123412341234".to_owned();
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_token));

        let save_user = user.save(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: save_user.username.unwrap(),
            id: save_user.id.unwrap(),
            token: save_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}