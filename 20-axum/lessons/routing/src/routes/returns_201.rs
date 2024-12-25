use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, ()).into_response()
    // (StatusCode::CREATED, "This is a 201".to_owned()).into_response()
}
