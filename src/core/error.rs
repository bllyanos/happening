use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::{auth::error::AuthServiceError, users::error::UserServiceError};

pub enum AppError {
    UserService(UserServiceError),
    AuthService(AuthServiceError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use AppError::*;

        let (status, error_message) = match self {
            UserService(inner) => inner.get_status_and_message(),
            AuthService(inner) => inner.get_status_and_message(),
        };

        let body = Json(json!({ "error_message": error_message }));
        (status, body).into_response()
    }
}
