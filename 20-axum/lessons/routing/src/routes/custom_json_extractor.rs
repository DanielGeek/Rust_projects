use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Json},
    http::StatusCode,
    response::IntoResponse,
    BoxError,
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

#[async_trait]
impl<B> FromRequest<B, axum::body::Body> for RequestUser
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(
        req: axum::http::Request<axum::body::Body>,
        _state: &B,
    ) -> Result<Self, Self::Rejection> {
        let Json(user): Json<RequestUser> = Json::<RequestUser>::from_request(req, &())
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}", err)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(Json(user): Json<RequestUser>) -> impl IntoResponse {
    dbg!(user);
}
