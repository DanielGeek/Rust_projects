use axum::{
    body::{Body, HttpBody},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header<B>(
    request: Request<B>,
    next: Next,
) -> Result<Response, StatusCode>
where
    B: HttpBody<Data = axum::body::Bytes> + Send + 'static,
    B::Error: Into<axum::BoxError>,
{
    let (parts, body) = request.into_parts();

    let message = parts
        .headers
        .get("message")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_string();

    let body = Body::new(body);

    let mut request = Request::from_parts(parts, body);
    request.extensions_mut().insert(HeaderMessage(message));

    Ok(next.run(request).await)
}
