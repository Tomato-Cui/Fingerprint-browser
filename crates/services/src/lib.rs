pub mod error;

pub mod command;
pub mod environment;
pub mod fingerprint;
pub mod group;
pub mod proxy;
pub mod resource_whitelist;
pub mod user;

#[allow(dead_code)]
pub(crate) async fn setup() {
    let mut migration_path = std::env::current_dir().unwrap();
    migration_path.pop();
    migration_path.pop();

    states::database::init_sqlite_database().await.unwrap();
}
