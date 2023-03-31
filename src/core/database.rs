use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(15)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgres://postgres:admin@localhost:5432/postgres/public")
        .await
}
