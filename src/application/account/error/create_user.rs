use crate::domain::repository::error::user::Error as UserRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create user: {0}")]
    UserRepositoryError(#[from] UserRepositoryError),
}
