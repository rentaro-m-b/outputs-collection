use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::ActiveValue::Set;

use crate::domain::repository::user::UserRepository;
use crate::domain::model::user::ActiveModel as UserActiveModel;
use crate::application::account::error::create_user::Error as CreateUserApplicationError;

#[async_trait]
pub trait CreateUserUsecase: Sync + Send {
    async fn create_user(&self, username: String, email: String, password: String) -> Result<UserActiveModel, CreateUserApplicationError>;
}

pub struct CreateUserUsecaseImpl {
    pub user_repository: Arc<dyn UserRepository + Sync + Send>
}

#[async_trait]
impl CreateUserUsecase for CreateUserUsecaseImpl {
    async fn create_user(&self, username: String, email: String, password: String) -> Result<UserActiveModel, CreateUserApplicationError> {
        let user = UserActiveModel{
            name: Set(username),
            email: Set(email),
            password: Set(password),
            ..Default::default()
        };
        self.user_repository.save(user.clone()).await?;
        Ok(user)
    }
}

