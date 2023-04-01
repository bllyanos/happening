pub enum UserServiceError {
    NotFound,
    UnknownSqlx(sqlx::Error),
}
