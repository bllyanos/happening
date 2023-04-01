use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResult {
    pub username: String,
    pub token: String,
}

#[derive(Deserialize, Default)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
}
