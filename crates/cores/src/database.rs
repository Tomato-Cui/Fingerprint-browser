use sqlx::{migrate::Migrator, Pool, Sqlite};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::errors::ApplicationServerError;
use crate::Result;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database(Pool<Sqlite>);

impl Database {
    pub async fn new(url: &str) -> core::result::Result<Self, sqlx::Error> {
        Ok(Database(sqlx::sqlite::SqlitePool::connect(url).await?))
    }

    pub async fn migrator(migration_location: PathBuf) -> Result<()> {
        Ok(Migrator::new(migration_location)
            .await?
            .run(get_db()?)
            .await?)
    }

    pub fn get(&self) -> &Pool<Sqlite> {
        &self.0
    }
}

static DATABASE: OnceCell<Arc<Database>> = OnceCell::const_new();

pub async fn init_database(url: &str) -> Result<&'static Arc<Database>> {
    Ok(DATABASE
        .get_or_init(|| async {
            let database = Database::new(url).await.unwrap();

            Arc::new(database)
        })
        .await)
}

pub fn get_db() -> Result<&'static Pool<Sqlite>> {
    match DATABASE.get() {
        Some(db) => Ok(db.get()),
        None => Err(ApplicationServerError::DatabaseGetError),
    }
}
