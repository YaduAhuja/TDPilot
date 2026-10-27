use thiserror::Error;

#[derive(Debug, Error)]
pub enum TDAppError {
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Application error: {0}")]
    TDError(#[from] TDCustomError)
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct TDCustomError {
    pub message: &'static str
}