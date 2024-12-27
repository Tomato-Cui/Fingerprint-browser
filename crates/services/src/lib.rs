pub mod error;

pub mod command;
pub mod environment;
pub mod environment_account;
pub mod environment_fingerprint;
pub mod environment_group;
pub mod environment_proxy;
pub mod environment_proxy_group;
pub mod environment_transfer_history;
pub mod environment_trash;
pub mod environmnet_cookie;
pub mod resource_whitelist;
pub mod team;
pub mod team_group;
pub mod user;
pub mod user_team_temp;

#[allow(dead_code)]
pub(crate) async fn setup() {
    states::init_config_state(r#"../../config.toml"#).await;
    let mut migration_path = std::env::current_dir().unwrap();
    migration_path.pop();
    migration_path.pop();

    // let migration_path = migration_path.join("migrations");
    states::database::init_sqlite_database().await.unwrap();

    // let pool = states::database::get_database_pool().unwrap();
    // commons::database::Database::migrator(pool, migration_path)
    //     .await
    //     .unwrap();
}

#[tokio::test]
async fn test_setup() {
    setup().await;
}
