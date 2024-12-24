pub mod environment;
pub mod fingerprint;
pub mod group;
pub mod proxies;
pub mod resource_whitelist;
pub mod team;
pub mod user;

#[allow(dead_code)]
pub(crate) async fn setup() {
    states::init_config_state(r#"../../config.toml"#).await;
    let mut migration_path = std::env::current_dir().unwrap();
    migration_path.pop();
    migration_path.pop();

    let migration_path = migration_path.join("migrations");
    states::database::init_sqlite_database().await.unwrap();
    println!("{:?}", migration_path);

    let pool = states::database::get_database_pool().unwrap();
    commons::database::Database::migrator(pool, migration_path)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_setup() {
    setup().await;
}
