use async_trait::async_trait;
use jsonwebtoken::TokenData;

use super::service::authentication::{validate_token, Claims};
use crate::application::account::error::login::Error as LoginApplicationError;

#[async_trait]
pub trait LoginUsecase: Sync + Send {
    async fn login(&self, token: &str, secret_key: &str) -> Result<TokenData<Claims>, LoginApplicationError>;
}

pub struct LoginUsecaseImpl {}

#[async_trait]
impl LoginUsecase for LoginUsecaseImpl {
    async fn login(&self, token: &str, secret_key: &str) -> Result<TokenData<Claims>, LoginApplicationError> {
        match validate_token(token, secret_key).await {
            Ok(token_data) => Ok(token_data),
            Err(e) => Err(LoginApplicationError::ValidateTokenError(e))
        }
    }
}
