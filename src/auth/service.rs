use sqlx::PgPool;

use crate::users::service::UserService;

use super::{
    error::AuthServiceError,
    model::{LoginInput, LoginResult},
    tokenization::{jwt_secret, tokenize_user},
};

#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
}

impl AuthService {
    pub fn new(db: PgPool) -> Self {
        // required
        jwt_secret();

        Self {
            user_service: UserService::new(db),
        }
    }

    pub async fn login(&self, input: LoginInput) -> Result<LoginResult, AuthServiceError> {
        let user = self
            .user_service
            .get_one_by_username(&input.username)
            .await?;

        if &input.password != &user.password {
            return Err(AuthServiceError::Unauthorized);
        }

        let token = tokenize_user(jwt_secret(), &user);

        Ok(LoginResult {
            token,
            username: String::from(&user.username),
        })
    }
}
