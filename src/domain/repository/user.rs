use async_trait::async_trait;

use crate::domain::model::user::ActiveModel as UserActiveModel;
use crate::domain::repository::error::user::Error as UserRepositoryError;

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn save(&self, user: UserActiveModel) -> Result<(), UserRepositoryError>;
}
