use sqlx::PgPool;

use crate::users::service::UserService;

use super::{
    error::AuthServiceError,
    model::{LoginInput, LoginResult},
};

pub struct AuthService {
    db: PgPool,
    user_service: UserService,
}

impl AuthService {
    pub fn new(db: PgPool) -> Self {
        Self {
            db: db.clone(),
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

        Ok(LoginResult {
            token: todo!("token woi"),
            username: String::from(&user.username),
        })
    }
}
