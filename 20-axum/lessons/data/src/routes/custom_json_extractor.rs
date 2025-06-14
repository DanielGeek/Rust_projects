use axum::{
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "Must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "Must have at least 8 characters"))]
    pub password: String,
}

pub async fn custom_json_extractor(
    Json(user): Json<RequestUser>,
) -> Result<(), (StatusCode, String)> {
    if let Err(errors) = user.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }
    dbg!(user);
    Ok(())
}
