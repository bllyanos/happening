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
    let bearer_token = headers
        .get("Authorization")
        .ok_or_else(|| AuthServiceError::InvalidToken)?;

    let bearer_token: Vec<&str> = bearer_token
        .to_str()
        .map_err(|_| AuthServiceError::InvalidToken)?
        .split_whitespace()
        .collect();

    let &bearer_token = bearer_token
        .get(1)
        .ok_or_else(|| AuthServiceError::InvalidToken)?;

    let result = verify(jwt_secret(), bearer_token)?;

    let result = LoginResult {
        username: result,
        token: String::from(bearer_token),
    };

    Ok(Json(result))
}

async fn handle_login(
    State(auth_service): State<AuthService>,
    Json(input): Json<LoginInput>,
) -> Result<Json<LoginResult>, AppError> {
    Ok(Json(auth_service.login(input).await?))
}
