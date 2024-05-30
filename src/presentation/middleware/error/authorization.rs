use std::convert::Infallible;

use crate::application::account::error::authorization::Error as AuthorizationApplicationError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to authentication service: {0}")]
    AuthorizationApplicationError(#[from] AuthorizationApplicationError),

    #[error("Failed to call on inner: {0}")]
    Other(String),
}
