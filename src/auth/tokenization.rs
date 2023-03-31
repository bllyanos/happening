use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::users::model::User;

use super::error::AuthServiceError;

static mut JWT_SECRET: String = String::new();

pub fn jwt_secret() -> &'static str {
    unsafe {
        if JWT_SECRET == "" {
            JWT_SECRET = dotenv::var("JWT_SECRET").expect("JWT_SECRET not set");
        }
        &JWT_SECRET
    }
}

pub fn tokenize_user(key: &str, user: &User) -> String {
    let key: Hmac<Sha256> = create_hmac_key(key);
    let mut claims = BTreeMap::new();
    claims.insert("username", &user.username);
    claims.sign_with_key(&key).expect("cannot sign the payload")
}

pub fn verify(key: &str, token: &str) -> Result<String, AuthServiceError> {
    let key = create_hmac_key(key);
    let claims: BTreeMap<String, String> = token
        .verify_with_key(&key)
        .map_err(|_| AuthServiceError::InvalidToken)?;
    claims
        .get("username")
        .ok_or_else(|| AuthServiceError::InvalidToken)
        .map(|username| username.to_owned())
}

fn create_hmac_key(key: &str) -> Hmac<Sha256> {
    let byte_key: &[u8] = key.as_bytes();
    Mac::new_from_slice(byte_key).expect("jwt key creation error")
}

#[test]
fn test_tokenize_user() {
    let key = "randommmm";
    let mut user = User::default();
    user.username = String::from("hello");

    let token = tokenize_user(key, &user);
    let verify_result = verify(key, &token);

    let mut unwrapped_username = String::new();

    if let Ok(username) = verify_result {
        unwrapped_username = username;
    }

    assert_eq!(unwrapped_username, String::from(&user.username));
}
