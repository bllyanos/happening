use crate::users::error::UserServiceError;

pub enum AuthServiceError {
    NotFound,
    Unauthorized,
    UnknownSqlx(sqlx::Error),
    Unknown,
}

impl From<UserServiceError> for AuthServiceError {
    fn from(value: UserServiceError) -> Self {
        match value {
            UserServiceError::NotFound => Self::NotFound,
            UserServiceError::UnknownSqlx(inner) => Self::UnknownSqlx(inner),
        }
    }
}
