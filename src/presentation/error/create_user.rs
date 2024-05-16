use crate::application::account::error::create_user::Error as CreateUserApplicationError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create user: {0}")]
    CreateUserPresentationError(#[from] CreateUserApplicationError),
}
