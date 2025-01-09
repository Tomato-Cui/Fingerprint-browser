pub mod requests;

#[allow(dead_code)]
pub async fn setup() {
    states::init_config_state(r#"../../config.toml"#).await;
    let mut migration_path = std::env::current_dir().unwrap();
    migration_path.pop();
    migration_path.pop();

    let _migration_path = migration_path.join("migrations");
    states::database::init_sqlite_database().await.unwrap();

    states::init_state().await;

    // let pool = states::database::get_database_pool().unwrap();
    // commons::database::Database::migrator(pool).await.unwrap();
}

#[tokio::test]
async fn test_setup() {
    setup().await;
}
