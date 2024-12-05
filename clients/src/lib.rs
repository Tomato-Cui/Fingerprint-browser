static CONFIG: &str = include_str!("../.././config.toml");

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    use cores::database;
    use cores::database::Database;

    // init config
    // cores::config::init_config(r#"config.toml"#).await;
    cores::config::init_config_by_str(CONFIG).await;

    // init location before db
    cores::init_location(cores::config::get_config()?.get_locations().await?).await?;
    // init browser version info, WARN: move other where
    // cores::requests::browser_resources::chrome::init_action_client(url);

    // migrates
    database::init_database(&cores::config::get_config()?.cache.name)
        .await
        .unwrap();
    Database::migrator(cores::config::get_config()?.cache.migrate_location.clone()).await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5678").await?;
    let routes = server::build_root_router();

    axum::serve(listener, routes).await?;
    Ok(())
}
