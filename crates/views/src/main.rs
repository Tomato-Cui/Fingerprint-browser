#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

static CONFIG: &str = include_str!("../../../config.toml");

use commons::config::Location;
use std::{path::PathBuf, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init config
    // states::init_config_state(r#"config.toml"#).await;
    states::init_config_state_str(CONFIG).await;

    let config = states::config::get_config().expect("loading config failed.");

    let Location {
        user_data_location,
        user_logs_location,
        browser_drivers_location,
        browser_extensions_location,
    } = config.app.location.clone();
    let database_location = config
        .database
        .location
        .clone()
        .unwrap_or_else(|| "cache".to_string());

    let locations = vec![
        PathBuf::from_str(&user_data_location).unwrap(),
        PathBuf::from_str(&user_logs_location).unwrap(),
        PathBuf::from_str(&browser_drivers_location).unwrap(),
        PathBuf::from_str(&browser_extensions_location).unwrap(),
        PathBuf::from_str(&browser_extensions_location).unwrap(),
        PathBuf::from_str(&database_location).unwrap(),
    ];
    // init cache locations
    cores::init_location(locations)
        .await
        .expect("loading cache location failed.");

    // init browser version info, WARN: move other where
    // cores::requests::browser_resources::chrome::init_action_client(url);

    // init states
    states::init_state().await;

    let pool = states::database::get_database_pool()?;
    if let Err(_) = commons::database::Database::migrator(pool).await {
        let cache_dir = states::config::APP_DATA.clone().join(database_location);
        if let Ok(_) = commons::util::delete_data_file(cache_dir).await {
            if let Err(_) = commons::database::Database::migrator(pool).await {
                eprintln!("database migrate failed.")
            }
        }
    }

    views::run();
    Ok(())
}
