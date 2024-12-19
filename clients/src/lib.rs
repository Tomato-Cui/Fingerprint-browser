// static CONFIG: &str = include_str!("../.././config.toml");

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // init config
    states::init_config_state(r#"config.toml"#).await;
    // cores::config::init_config_by_str(CONFIG).await;

    // init location before db
    // cores::init_location(cores::config::get_config()?.get_locations().await?).await?;
    // init browser version info, WARN: move other where
    // cores::requests::browser_resources::chrome::init_action_client(url);

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
