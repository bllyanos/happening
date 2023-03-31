use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub is_verified: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub verified_at: Option<chrono::DateTime<Utc>>,
}
