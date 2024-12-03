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

#[tokio::test]
async fn test_init_location() {
    use cores::config::get_config;
    use cores::init_location;

    init_config().await;

    init_location(get_config().unwrap().get_locations().await.unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn test_init_config() {
    init_config().await;
}
