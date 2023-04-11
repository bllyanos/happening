use axum::http::HeaderMap;

use crate::core::error::AppError;

use super::{
    error::AuthServiceError,
    model::LoginResult,
    tokenization::{jwt_secret, verify},
};

pub fn extract(headers: HeaderMap) -> Result<LoginResult, AppError> {
    let auth_header = headers
        .get("Authorization")
        .ok_or_else(|| AuthServiceError::InvalidToken)?;

    let auth_header_split: Vec<&str> = auth_header
        .to_str()
        .map_err(|_| AuthServiceError::InvalidToken)?
        .split_whitespace()
        .collect();

    let &bearer_token = auth_header_split
        .get(1)
        .ok_or_else(|| AuthServiceError::InvalidToken)?;

    let verify_result = verify(jwt_secret(), bearer_token)?;

    let login_result = LoginResult {
        username: verify_result,
        token: String::from(bearer_token),
    };

    Ok(login_result)
}
