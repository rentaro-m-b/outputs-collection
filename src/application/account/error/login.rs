use crate::domain::repository::error::user::Error as UserRepositoryError;
use crate::application::account::service::error::crypter::Error as CrypterServiceError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to crypter service: {0}")]
    CrypterServiceError(#[from] CrypterServiceError),

    #[error("Failed to user repository: {0}")]
    UserRepositoryError(#[from] UserRepositoryError)
}
