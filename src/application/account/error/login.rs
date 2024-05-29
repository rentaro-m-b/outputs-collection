use crate::domain::repository::error::user::Error as UserRepositoryError;
use crate::application::account::service::error::crypter::Error as CrypterServiceError;
use crate::application::account::service::error::authentication::Error as AuthenticationServiceError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to crypter service: {0}")]
    CrypterServiceError(#[from] CrypterServiceError),

    #[error("Failed to user repository: {0}")]
    UserRepositoryError(#[from] UserRepositoryError),

    #[error("Failed to authentication service: {0}")]
    AuthenticationServiceError(#[from] AuthenticationServiceError)
}
