use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResult {
    pub username: String,
    pub token: String,
}
