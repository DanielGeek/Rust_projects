use std::env;

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims { exp, iat };
    let secret: String = env::var("JWT_SECRET").unwrap();
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &key).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &String) -> Result<bool, AppError> {
    let secret: String = env::var("JWT_SECRET").unwrap();
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token.as_str(), &key, &Validation::new(Algorithm::HS256)).map_err(
        |error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(
                StatusCode::UNAUTHORIZED,
                "Your session has expired, please login again",
            ),
            _ => AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            ),
        },
    )?;
    Ok(true)
}
