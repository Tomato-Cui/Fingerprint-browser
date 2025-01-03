use sqlx::{Pool, Sqlite};

pub enum DatabaseDriverType {
    Sqlite,
    Postgresql,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database(Pool<Sqlite>);

impl Database {
    pub async fn new(
        driver_type: DatabaseDriverType,
        url: &str,
    ) -> Result<Self, sqlx::error::Error> {
        match driver_type {
            DatabaseDriverType::Sqlite => Ok(Database(
                sqlx::sqlite::SqlitePool::connect(&sqlite_url_build(url)).await?,
            )),
            DatabaseDriverType::Postgresql => {
                Ok(Database(sqlx::sqlite::SqlitePool::connect(url).await?))
            }
        }
    }

    pub async fn migrator(pool: &Pool<Sqlite>) -> Result<(), sqlx::migrate::MigrateError> {
        Ok(sqlx::migrate!("../../migrations").run(pool).await?)
    }

    pub fn get(&self) -> &Pool<Sqlite> {
        &self.0
    }
}

fn sqlite_url_build(url: &str) -> String {
    if url.contains(".db") {
        format!("sqlite://{}?mode=rwc", url)
    } else {
        format!("sqlite::{}:", url)
    }
}

#[tokio::test]
async fn test_database_sqlite_location() {
    let mut db_path = std::env::current_dir().unwrap();
    db_path.pop();
    db_path.pop();
    db_path = db_path.join("test.db");

    Database::new(
        DatabaseDriverType::Sqlite,
        db_path.as_os_str().to_str().unwrap(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_database_sqlite_memory() {
    Database::new(DatabaseDriverType::Sqlite, "memory")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_migrator() {
    // let mut migration_path = std::env::current_dir().unwrap();
    // migration_path.pop();
    // migration_path.pop();
    // migration_path = migration_path.join("migrations");

    let db_url = vec!["test.db", "memory"];

    for url in db_url {
        let pool = Database::new(DatabaseDriverType::Sqlite, url)
            .await
            .unwrap();

        Database::migrator(pool.get()).await.unwrap();
    }
}

#[tokio::test]
async fn test_migrator_2() {
    let mut db_path = std::env::current_dir().unwrap();
    db_path.pop();
    db_path.pop();
    // let migration_path = db_path.join("migrations");
    db_path = db_path.join("test.db");

    let pool = Database::new(
        DatabaseDriverType::Sqlite,
        db_path.as_os_str().to_str().unwrap(),
    )
    .await
    .unwrap();

    Database::migrator(pool.get()).await.unwrap();
}
