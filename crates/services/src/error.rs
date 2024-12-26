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
            sqlx::error::Error::Database(e) => {
                let message = e.message();
                if message.contains("UNIQUE constraint failed") {
                    let replaced_message = message.replace("UNIQUE constraint failed", "");
                    let mut message = replaced_message.split(".").clone();
                    message.next();
                    let colname_name = message.next();

                    ServiceError::Error(format!(
                        "{} 数据不能重复",
                        colname_name.unwrap_or_default()
                    ))
                } else {
                    ServiceError::Error(message.to_string())
                }
            }
            err => ServiceError::Error(format!("{}", err)),
        }
    }
}
