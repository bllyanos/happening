use axum::http::StatusCode;

pub enum UserServiceError {
    NotFound,
    UnknownSqlx(sqlx::Error),
}

impl UserServiceError {
    pub fn get_status_and_message(&self) -> (StatusCode, String) {
        match self {
            UserServiceError::NotFound => (StatusCode::NOT_FOUND, format!("user not found")),
            UserServiceError::UnknownSqlx(inner) => {
                println!("user_service:unknown_sql_error {:?}", inner);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("unknown database error"),
                )
            }
        }
    }
}
