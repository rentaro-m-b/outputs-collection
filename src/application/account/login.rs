use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::repository::user::UserRepository;
use crate::application::account::error::login::Error as LoginApplicationError;

use super::service::authentication::AuthenticationService;
use super::service::crypter::CrypterService;

#[async_trait]
pub trait LoginUsecase: Sync + Send {
    async fn login(&self, email: String, password: String) -> Result<String, LoginApplicationError>;
}

pub struct LoginUsecaseImpl {
    pub user_repository: Arc<dyn UserRepository + Sync + Send>,
    pub crypter_service: Arc<dyn CrypterService + Sync + Send>,
    pub authentication_service: Arc<dyn AuthenticationService + Sync + Send>,
}

#[async_trait]
impl LoginUsecase for LoginUsecaseImpl {
    async fn login(&self, email: String, password: String) -> Result<String, LoginApplicationError> {
        let user = self.user_repository.find_by_email(email).await?;
        let (user_id, password_hash) = match user {
            Some(user) => (user.id.to_string(), user.password),
            None => {
                return Ok("".to_owned());
            }
        };

        let is_validated = self.crypter_service.validate_password(&password, &password_hash).await?;
        if is_validated {
            let token = self.authentication_service.generate_token(user_id).await?;
            Ok(token)
        } else {
            Ok("".to_owned())
        }

    }
}
