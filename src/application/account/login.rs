use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{model::user::ActiveModel as UserActiveModel, model::user::Model as UserModel, repository::user::UserRepository};
use crate::application::account::error::login::Error as LoginApplicationError;

use super::service::crypter::validate_password;

#[async_trait]
pub trait LoginUsecase: Sync + Send {
    async fn login(&self, email: String, password: String) -> Result<UserActiveModel, LoginApplicationError>;
}

pub struct LoginUsecaseImpl {
    pub user_repository: Arc<dyn UserRepository + Sync + Send>
}

#[async_trait]
impl LoginUsecase for LoginUsecaseImpl {
    async fn login(&self, email: String, password: String) -> Result<UserActiveModel, LoginApplicationError> {
        let user = self.user_repository.find_by_email(email).await?;
        validate_password(&password).await
    }
}
