use std::sync::Arc;

use async_trait::async_trait;
use jsonwebtoken::TokenData;

use super::service::authentication::{AuthenticationService, Claims};
use crate::application::account::error::authorization::Error as AuthorizationApplicationError;

#[async_trait]
pub trait AuthorizationUsecase: Sync + Send {
    async fn authorize(&self, token: String) -> Result<TokenData<Claims>, AuthorizationApplicationError>;
}

pub struct AuthorizationUsecaseImpl {
    pub authentication_service: Arc<dyn AuthenticationService + Sync + Send>
}

#[async_trait]
impl AuthorizationUsecase for AuthorizationUsecaseImpl {
    async fn authorize(&self, token: String) -> Result<TokenData<Claims>, AuthorizationApplicationError> {
        let claims = self.authentication_service.validate_token(&token).await?;
        Ok(claims)
    }
}
