use async_trait::async_trait;
use jsonwebtoken::TokenData;

use super::service::authentication::{validate_token, Claims};
use crate::application::account::error::authorization::Error as AuthorizationApplicationError;

#[async_trait]
pub trait AuthorizationUsecase: Sync + Send {
    async fn login(&self, token: &str, secret_key: &str) -> Result<TokenData<Claims>, AuthorizationApplicationError>;
}

pub struct AuthorizationUsecaseImpl {}

#[async_trait]
impl AuthorizationUsecase for AuthorizationUsecaseImpl {
    async fn login(&self, token: &str, secret_key: &str) -> Result<TokenData<Claims>, AuthorizationApplicationError> {
        match validate_token(token, secret_key).await {
            Ok(token_data) => Ok(token_data),
            Err(e) => Err(AuthorizationApplicationError::ValidateTokenError(e))
        }
    }
}
