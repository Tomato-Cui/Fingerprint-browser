pub mod auth;
pub mod config;
pub mod database;
pub mod tauri;

pub async fn init_state() {
    auth::init_state().await;
    tauri::init_state().await;
    database::init_sqlite_database()
        .await
        .expect("database connect init failed.");
}

pub async fn init_config_state(path: &str) {
    config::init_state_path(path).await;
}
pub async fn init_config_state_str(config_str: &str) {
    config::init_state_str(config_str).await;
}

