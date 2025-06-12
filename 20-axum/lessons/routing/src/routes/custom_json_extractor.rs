use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
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

#[derive(Debug)]
pub struct ValidationError(pub String);

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

pub async fn custom_json_extractor(
    Json(user): Json<RequestUser>,
) -> Result<impl IntoResponse, ValidationError> {
    user.validate()
        .map_err(|e| ValidationError(format!("{}", e)))?;

    dbg!(user);
    Ok("User received successfully")
}
