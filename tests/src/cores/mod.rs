mod apis;
mod models;
mod requests;
mod test_config;
mod test_db;
mod utils;

pub async fn init_config() {
    use cores::config;

    config::init_config(r#"..\config.toml"#).await;
}

pub async fn load_db() {
    use cores::database::init_database;

    init_config().await;
    // init_database(&get_config().unwrap().cache.name)
    init_database("sqlite::memory:").await.unwrap();
}

#[tokio::test]
async fn test_init_location() {
    use cores::config::get_config;
    use cores::init_location;

    init_config().await;

    init_location(get_config().unwrap().get_locations().unwrap()).unwrap();
}

#[tokio::test]
async fn test_init_config() {
    init_config().await;
}

#[tokio::test]
async fn test_load_db() {
    load_db().await;
}
