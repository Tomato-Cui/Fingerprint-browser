use sqlx::sqlite;
use thiserror::Error;

use crate::utils::response::AppResponse;

#[derive(Error, Debug)]
pub enum ApplicationServerError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlite::SqliteError),

    #[error("Database execute error: {0}.")]
    DatabaseExecuteError(#[from] sqlx::Error),

    #[error("Database get error.")]
    DatabaseGetError,

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

    #[error("blockmodeerror error: {0}")]
    BlockModeError(#[from] block_modes::BlockModeError),

    #[error("Child Running Fail.")]
    ChildRunningError,

    #[error("Child Close Fail.")]
    ChildCloseError,

    #[error("ffi current null: {0}")]
    FFINullError(#[from] std::ffi::NulError),

    #[error("get processer fail.")]
    GetProcesserError,

    #[error("get processer fail.")]
    SystemTimeError(#[from] std::time::SystemTimeError),

    #[error("config load fail.")]
    ConfigLoadError,

    #[error("server fetch load fail: {0}")]
    ServerFetchFail(#[from] reqwest::Error),

    #[error("url parse fail.")]
    UrlParseFail,

    #[error("{0}")]
    Error(#[from] anyhow::Error),
}

impl<T> Into<AppResponse<T>> for ApplicationServerError {
    fn into(self) -> AppResponse<T> {
        AppResponse::fail(Some(self.to_string()))
    }
}
