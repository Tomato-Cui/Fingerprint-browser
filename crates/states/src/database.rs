use commons::database::{Database, DatabaseDriverType};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::config::APP_DATA;

static DATABASE: OnceCell<Arc<Database>> = OnceCell::const_new();

pub async fn init_sqlite_database() -> Result<&'static Arc<Database>, sqlx::error::Error> {
    let config = crate::config::get_config()
        .map(|v| v.get_database_config())
        .expect("config not found");

    let app_data = APP_DATA.clone();
    let cache_base_location = app_data.to_str().unwrap();
    let url = if config.url.contains(".db") {
        if let Some(location) = config.location.clone() {
            &format!(
                r#"{}/{}/{}"#,
                cache_base_location, location, &config.url
            )
        } else {
            &format!(r#"{}/cache/{}"#, cache_base_location, &config.url)
        }
    } else {
        &config.url
    };

    Ok(DATABASE
        .get_or_init(|| async {
            let database = Database::new(DatabaseDriverType::Sqlite, url)
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
