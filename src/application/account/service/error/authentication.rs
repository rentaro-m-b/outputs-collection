#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to get current time: {0}")]
    GetCurrentTimeError(String),

    #[error("Failed to decord token: {0}")]
    DecordTokenError(String),
}
