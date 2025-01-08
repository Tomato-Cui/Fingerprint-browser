pub mod allowed_operation;
pub mod codes;
pub mod environment;
pub mod environment_account;
pub mod environment_fingerprint;
pub mod environment_group;
pub mod environment_proxies;
pub mod environment_proxy_group;
pub mod environment_transfer_history;
pub mod environment_trash;
pub mod environmnet_cookie;
pub mod extension;
pub mod operation_log;
pub mod resource_whitelist;
pub mod team;
pub mod team_group;
pub mod team_group_permission;
pub mod user;
pub mod user_avatar;
pub mod user_info;
pub mod user_team_temp;
pub mod user_use_team;

pub mod dto;

#[allow(dead_code)]
pub(crate) async fn setup() {
    states::init_config_state(r#"../../config.toml"#).await;
    let mut migration_path = std::env::current_dir().unwrap();
    migration_path.pop();
    migration_path.pop();

    let _migration_path = migration_path.join("migrations");
    states::database::init_sqlite_database().await.unwrap();
    println!("{:?}", migration_path);

    let pool = states::database::get_database_pool().unwrap();
    commons::database::Database::migrator(pool).await.unwrap();
}

#[tokio::test]
async fn test_setup() {
    setup().await;
}
