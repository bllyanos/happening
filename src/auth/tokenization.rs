use std::time::{SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use jwt::{Claims, RegisteredClaims, SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::{core::env_var::env_var, users::model::User};

use super::error::AuthServiceError;

static mut JWT_SECRET: String = String::new();
const JWT_EXP_SECS: u64 = 86400;

pub fn jwt_secret() -> &'static str {
    unsafe {
        if JWT_SECRET == "" {
            JWT_SECRET = env_var("JWT_SECRET")
        }
        &JWT_SECRET
    }
}

pub fn tokenize_user(key: &str, user: &User) -> String {
    let key: Hmac<Sha256> = create_hmac_key(key);
    let mut claims = Claims::new(get_reg_claims());
    claims.private.insert(
        String::from("username"),
        serde_json::Value::String(String::from(&user.username)),
    );
    claims.sign_with_key(&key).expect("cannot sign the payload")
}

pub fn verify(key: &str, token: &str) -> Result<String, AuthServiceError> {
    let key = create_hmac_key(key);
    let claims: Claims = token
        .verify_with_key(&key)
        .map_err(|_| AuthServiceError::InvalidToken)?;
    claims
        .private
        .get("username")
        .ok_or_else(|| AuthServiceError::InvalidToken)
        .map(|username| username.as_str().unwrap().to_owned())
}

fn get_expiration() -> u64 {
    let now: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("invalid system time")
        .as_secs()
        .try_into()
        .unwrap();
    return now + JWT_EXP_SECS;
}

fn get_reg_claims() -> RegisteredClaims {
    let mut reg_claims = RegisteredClaims::default();
    reg_claims.expiration = Some(get_expiration());
    reg_claims
}

fn create_hmac_key(key: &str) -> Hmac<Sha256> {
    Mac::new_from_slice(key.as_bytes() as &[u8]).expect("jwt key creation error")
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
