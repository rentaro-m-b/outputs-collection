use crate::application::account::service::error::authentication::Error as AuthenticationServiceError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to login: {0}")]
    ValidateTokenError(#[from] AuthenticationServiceError)
}
