#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to hash password: {0}")]
    HashPasswordError(String),
}
