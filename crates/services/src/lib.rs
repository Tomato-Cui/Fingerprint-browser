pub mod error;

pub mod command;
pub mod environment;
pub mod environment_trash;
pub mod fingerprint;
pub mod group;
pub mod proxy;
pub mod resource_whitelist;
pub mod team;
pub mod user;

#[allow(dead_code)]
pub(crate) async fn setup() {
    states::init_config_state(r#"../../config.toml"#).await;
    let mut migration_path = std::env::current_dir().unwrap();
    migration_path.pop();
    migration_path.pop();

    states::database::init_sqlite_database().await.unwrap();
}
