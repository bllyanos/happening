use axum::http::StatusCode;

use crate::{core::error::AppError, users::error::UserServiceError};

pub enum AuthServiceError {
    NotFound,
    Unauthorized,
    InvalidToken,
    UnknownSqlx(sqlx::Error),
}

impl AuthServiceError {
    pub fn get_status_and_message(&self) -> (axum::http::StatusCode, String) {
        match &self {
            AuthServiceError::NotFound => (StatusCode::NOT_FOUND, format!("user not found")),
            AuthServiceError::Unauthorized => (StatusCode::UNAUTHORIZED, format!("unauthorized")),
            AuthServiceError::InvalidToken => (StatusCode::FORBIDDEN, format!("invalid token")),
            AuthServiceError::UnknownSqlx(inner) => {
                println!("auth_service:unknown_sql_error {:?}", inner);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("unknown database error"),
                )
            }
        }
    }
}

impl From<UserServiceError> for AuthServiceError {
    fn from(value: UserServiceError) -> Self {
        match value {
            UserServiceError::NotFound => Self::NotFound,
            UserServiceError::UnknownSqlx(inner) => Self::UnknownSqlx(inner),
        }
    }
}

impl From<AuthServiceError> for AppError {
    fn from(value: AuthServiceError) -> Self {
        AppError::AuthService(value)
    }
}
