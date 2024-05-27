use std::sync::Arc;
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use async_trait::async_trait;

use crate::domain::repository::user::UserRepository;
use crate::application::account::service::error::crypter::Error as CrypterServiceError;

#[async_trait]
pub trait CrypterService: Sync + Send {
    async fn hash_password(&self, password: &str) -> Result<String, CrypterServiceError>;
    async fn validate_password(&self, password: &str, password_hash: &str) -> Result<bool, CrypterServiceError>;
}

pub struct CrypterServiceImpl {
    pub user_repository: Arc<dyn UserRepository + Sync + Send>,
}

#[async_trait]
impl CrypterService for CrypterServiceImpl {
    async fn hash_password(&self, password: &str) -> Result<String, CrypterServiceError> {
        let password_bytes = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = match argon2.hash_password(password_bytes, &salt) {
            Ok(value) => value.to_string(),
            Err(e) => return Err(CrypterServiceError::HashPasswordError(e.to_string()))
        };

        Ok(password_hash)
    }
    
    // パスワードとパスワードハッシュ（User.password）の整合を検証
    async fn validate_password(&self, password: &str, password_hash: &str) -> Result<bool, CrypterServiceError> {
        let password_bytes = password.as_bytes();
        let parsed_password_hash = match PasswordHash::new(password_hash) {
            Ok(value) => value,
            Err(e) => return Err(CrypterServiceError::HashPasswordError(e.to_string()))
        };
        
        Ok(Argon2::default().verify_password(password_bytes, &parsed_password_hash).is_ok())
    }    
}
