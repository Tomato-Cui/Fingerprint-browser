use commons::database::{Database, DatabaseDriverType};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::OnceCell;

static DATABASE: OnceCell<Arc<Database>> = OnceCell::const_new();

pub async fn init_sqlite_database() -> Result<&'static Arc<Database>, sqlx::error::Error> {
    let config = crate::config::get_config()
        .map(|v| v.get_database_config())
        .expect("config not found");

    Ok(DATABASE
        .get_or_init(|| async {
            let database = Database::new(DatabaseDriverType::Sqlite, &config.url)
                .await
                .unwrap();

            Arc::new(database)
        })
        .await)
}

pub fn get_database_pool() -> Result<&'static Pool<Sqlite>, sqlx::error::Error> {
    match DATABASE.get() {
        Some(db) => Ok(db.get()),
        None => Err(sqlx::error::Error::PoolClosed),
    }
}
