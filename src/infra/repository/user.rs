use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};

use crate::domain::model::user::{ActiveModel, Model as UserModel};
use crate::domain::repository::user::UserRepository as DomainUserRepository;
use crate::domain::repository::error::user::Error as DomainUserRepositoryError;

pub struct UserRepository {
    conn: Arc<DatabaseConnection>
}

#[async_trait]
impl DomainUserRepository for UserRepository {
    async fn save(&self, user: UserModel) -> Result<(), DomainUserRepositoryError> {
        user.into_active_model().save(self.conn.as_ref()).await.map_err(|e| 
            DomainUserRepositoryError::FailedToSaveDataToDb(e.to_string())
        );
        Ok(())
    }
}


