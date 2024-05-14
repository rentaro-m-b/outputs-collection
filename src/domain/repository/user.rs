use async_trait::async_trait;

use crate::domain::model::user::{ActiveModel, Model as UserModel};
use crate::domain::repository::error::user::Error as UserRepositoryError;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: UserModel) -> Result<(), UserRepositoryError>;
}
