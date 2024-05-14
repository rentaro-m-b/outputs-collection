use std::sync::Arc;

use crate::domain::repository::user::UserRepository;
use crate::domain::model::user::{ActiveModel, Model as UserModel};
use crate::application::account::error::create_user::Error as CreateUserApplicationError;

pub struct CreateUser {
    user_repository: Arc<dyn UserRepository>
}

impl CreateUser {
    pub async fn creat_user(&self, user: UserModel) -> Result<UserModel, CreateUserApplicationError> {
        self.user_repository.save(user.clone()).await?;
        Ok(user)
    }
}

