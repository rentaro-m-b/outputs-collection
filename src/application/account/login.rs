use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::repository::user::UserRepository;
use crate::application::account::error::login::Error as LoginApplicationError;

use super::service::crypter::CrypterService;

#[async_trait]
pub trait LoginUsecase: Sync + Send {
    async fn login(&self, email: String, password: String) -> Result<bool, LoginApplicationError>;
}

pub struct LoginUsecaseImpl {
    pub user_repository: Arc<dyn UserRepository + Sync + Send>,
    pub crypter_service: Arc<dyn CrypterService + Sync + Send>,
}

#[async_trait]
impl LoginUsecase for LoginUsecaseImpl {
    async fn login(&self, email: String, password: String) -> Result<bool, LoginApplicationError> {
        if let Some(user) = self.user_repository.find_by_email(email).await? {
            match self.crypter_service.validate_password(&password, &user.password).await {
                Ok(is_validated) => Ok(is_validated),
                Err(e) => Err(LoginApplicationError::CrypterServiceError(e))
            }
        } else {
            return Ok(false);
        }
    }
}
