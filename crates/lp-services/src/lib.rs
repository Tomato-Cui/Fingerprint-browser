pub mod error;

pub mod allowed_operation;
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
pub mod extension;
pub mod operation_log;
pub mod permission;
pub mod resource_whitelist;
pub mod team;
pub mod team_group;
pub mod user;
pub mod user_team_temp;

#[allow(dead_code)]
pub async fn setup() {
    lp_states::init_config_state(r#"../../config.toml"#).await;
    let mut migration_path = std::env::current_dir().unwrap();
    migration_path.pop();
    migration_path.pop();

    let _migration_path = migration_path.join("migrations");
    lp_states::database::init_sqlite_database().await.unwrap();

    lp_states::init_state().await;

    // let pool = lp_states::database::get_database_pool().unwrap();
    // lp_commons::database::Database::migrator(pool).await.unwrap();
}

#[tokio::test]
async fn test_setup() {
    setup().await;
}
