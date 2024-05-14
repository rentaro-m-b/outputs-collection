use crate::domain::repository::error::user::Error as DomainUserRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create user: {0}")]
    DomainUserRepositoryError(#[from] DomainUserRepositoryError),
}
