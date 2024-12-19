use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("database busy.")]
    DatabasePoolFetchError,

    #[error("{0}")]
    Error(String),
}

impl From<sqlx::error::Error> for ServiceError {
    fn from(value: sqlx::error::Error) -> Self {
        match value {
            sqlx::Error::PoolClosed => ServiceError::DatabasePoolFetchError,
            sqlx::Error::ColumnNotFound(e) => ServiceError::Error(format!("{e} column not found")),
            sqlx::Error::RowNotFound => ServiceError::Error(format!("row not found")),
            sqlx::error::Error::Database(e) => ServiceError::Error(e.message().to_string()),
            err => ServiceError::Error(err.to_string()),
        }
    }
}
