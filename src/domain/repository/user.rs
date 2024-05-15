use async_trait::async_trait;

use crate::domain::model::user::Model as UserModel;
use crate::domain::repository::error::user::Error as UserRepositoryError;

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn save(&self, user: UserModel) -> Result<(), UserRepositoryError>;
}
