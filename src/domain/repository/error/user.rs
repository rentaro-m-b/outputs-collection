#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to save data from database: {0}")]
    FailedToSaveDataToDb(String),
    #[error("Failed to find user from database: {0}")]
    FailedToFIndUserToDb(String),
}
