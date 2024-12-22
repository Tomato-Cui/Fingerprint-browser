// static CONFIG: &str = include_str!("../.././config.toml");

use std::{path::PathBuf, str::FromStr};

use commons::config::Location;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // init config
    states::init_config_state(r#"config.toml"#).await;
    // cores::config::init_config_by_str(CONFIG).await;

    let config = states::config::get_config().unwrap();
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
    cores::init_location(locations).await?;

    // init browser version info, WARN: move other where
    // cores::requests::browser_resources::chrome::init_action_client(url);

    // init states
    states::init_state().await;

    // migrates
    let migration_path = std::env::current_dir().unwrap().join("migrations");
    let pool = states::database::get_database_pool()?;
    commons::database::Database::migrator(pool, migration_path)
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5678").await?;
    let routes = apis::build_root_router();

    axum::serve(listener, routes).await?;
    Ok(())
}
