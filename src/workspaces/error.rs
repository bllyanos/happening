use axum::http::StatusCode;

use crate::core::error::AppError;

pub enum WorkspaceServiceError {
    NotFound,
    UnknownSqlx(sqlx::Error),
}

impl WorkspaceServiceError {
    pub fn get_status_and_message(&self) -> (StatusCode, String) {
        match &self {
            WorkspaceServiceError::NotFound => {
                (StatusCode::NOT_FOUND, format!("workspace not found"))
            }
            WorkspaceServiceError::UnknownSqlx(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("unknown database error"),
            ),
        }
    }
}

impl From<WorkspaceServiceError> for AppError {
    fn from(value: WorkspaceServiceError) -> Self {
        AppError::WorkspaceService(value)
    }
}
