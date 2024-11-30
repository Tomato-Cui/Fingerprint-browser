#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_file(false)
        .init();

    // init config
    cores::config::init_config(r#"config.toml"#).await;

    // init location before db
    cores::init_location(cores::config::get_config()?.get_locations()?)?;

    // init db
    cores::db::Db::new(cores::config::get_config()?.get_cache_location()?)?.init_db_structure()?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5678").await?;
    let routes = server::build_root_router();

    axum::serve(listener, routes).await?;
    Ok(())
}
