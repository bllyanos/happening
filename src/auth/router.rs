use axum::http::HeaderMap;
use axum::routing::*;
use axum::{extract::State, Json, Router};
use sqlx::PgPool;

use crate::core::error::AppError;

use super::bearer_token_header::extract;
use super::model::{LoginInput, LoginResult};
use super::service::AuthService;

pub fn create(db: PgPool) -> Router {
    let auth_service = AuthService::new(db);
    Router::new()
        .route("/auth", get(handle_verify))
        .route("/auth", post(handle_login))
        .with_state(auth_service)
}

async fn handle_verify(headers: HeaderMap) -> Result<Json<LoginResult>, AppError> {
    Ok(Json(extract(headers)?))
}

async fn handle_login(
    State(auth_service): State<AuthService>,
    Json(input): Json<LoginInput>,
) -> Result<Json<LoginResult>, AppError> {
    Ok(Json(auth_service.login(input).await?))
}
