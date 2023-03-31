use sqlx::PgPool;

use super::{error::UserServiceError, model::User};

#[derive(Clone)]
pub struct UserService {
    db: PgPool,
}

impl UserService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_one_by_username(&self, username: &str) -> Result<User, UserServiceError> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.db)
            .await
            .map_err(|err| UserServiceError::UnknownSqlx(err))?
            .ok_or_else(|| UserServiceError::NotFound)
    }
}
