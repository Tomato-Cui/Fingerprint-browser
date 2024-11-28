mod apis;
mod test_config;
mod test_db;
mod utils;

use cores::config::{self, get_config};

pub async fn init_config() {
    config::init_config(r#"..\config.toml"#).await;
}

pub async fn load_db() -> cores::db::Db {
    init_config().await;
    cores::db::Db::new(get_config().unwrap().get_cache_location().unwrap()).unwrap()
}

#[tokio::test]
async fn test_init_location() {
    use cores::init_location;

    init_config().await;

    init_location(get_config().unwrap().get_locations().unwrap()).unwrap();
}

#[tokio::test]
async fn test_init_config() {
    init_config().await;
}
