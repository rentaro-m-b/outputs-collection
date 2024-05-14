#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to save data from database: {0}")]
    FailedToSaveDataToDb(String),
}
