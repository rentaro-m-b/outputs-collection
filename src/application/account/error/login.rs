#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to login: {0}")]
    ValidateTokenError(#[from] jsonwebtoken::errors::Error)
}
