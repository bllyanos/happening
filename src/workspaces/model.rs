use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Workspace {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateWorkspace {
    pub owner_id: i64,
    pub name: String,
    pub slug: String,
}
