use axum::http::HeaderMap;
use axum::routing::*;
use axum::{extract::State, Json, Router};
use sqlx::PgPool;

use crate::core::error::AppError;

use super::error::AuthServiceError;
use super::model::{LoginInput, LoginResult};
use super::service::AuthService;
use super::tokenization::{jwt_secret, verify};

pub fn create(db: PgPool) -> Router {
    let auth_service = AuthService::new(db);
    Router::new()
        .route("/auth", get(handle_verify))
        .route("/auth", post(handle_login))
        .with_state(auth_service)
}

async fn handle_verify(headers: HeaderMap) -> Result<Json<LoginResult>, AppError> {
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

    Ok(Json(login_result))
}

async fn handle_login(
    State(auth_service): State<AuthService>,
    Json(input): Json<LoginInput>,
) -> Result<Json<LoginResult>, AppError> {
    Ok(Json(auth_service.login(input).await?))
}
