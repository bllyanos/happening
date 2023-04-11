use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

use super::env_vars::env_var;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let url = env_var("DATABASE_URL");
    PgPoolOptions::new()
        .max_connections(15)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await
}
