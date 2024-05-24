use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};

use crate::domain::model::user;
use crate::domain::model::user::ActiveModel as UserActiveModel;
use crate::domain::model::user::Model as UserModel;
use crate::domain::repository::user::UserRepository;
use crate::domain::repository::error::user::Error as UserRepositoryError;

pub struct UserRepositoryImpl {
    pub conn: Arc<DatabaseConnection>
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: UserActiveModel) -> Result<(), UserRepositoryError> {
        let _ = user.into_active_model()
            .save(self.conn.as_ref())
            .await
            .map_err(|e| 
                UserRepositoryError::FailedToSaveDataToDb(e.to_string())
            );
        Ok(())
    }

    async fn find_by_email(&self, email: String) -> Result<Option<UserModel>, UserRepositoryError> {
        let user = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(self.conn.as_ref())
            .await
            .map_err(|e| 
                UserRepositoryError::FailedToFIndUserToDb(e.to_string())
            );
        user
    }
}
