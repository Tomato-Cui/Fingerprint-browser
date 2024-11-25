use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationServerError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("base64 error: {0}")]
    B64DecodeError(#[from] base64::DecodeError),

    #[error("FromUtf8Error error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("InvalidKeyIvLength error: {0}")]
    InvalidKeyIvLength(#[from] block_modes::InvalidKeyIvLength),

    #[error("BlockModeError error: {0}")]
    BlockModeError(#[from] block_modes::BlockModeError),
}
