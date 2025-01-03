use axum::{http::StatusCode, middleware, routing::get, Router};
use commons::config::Location;
use std::{path::PathBuf, str::FromStr};

pub mod middlewares;
pub mod response;
pub mod routes;

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "")
}

pub fn build_root_router() -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .merge(routes::user::build_router())
                .merge(routes::browser::build_router())
                .merge(routes::environment_account::build_router())
                .merge(routes::environment_cookie::build_router())
                .merge(routes::environment_fingerprint::build_router())
                .merge(routes::environment_group::build_router())
                .merge(routes::environment_proxies::build_router())
                .merge(routes::environment_proxy_group::build_router())
                .merge(routes::environment_transfer_history::build_router())
                .merge(routes::environment_trash::build_router())
                .merge(routes::environment::build_router())
                .merge(routes::team_group::build_router())
                .merge(routes::team::build_router())
                .merge(routes::user_team_temp::build_router())
                .merge(
                    Router::new()
                        .route("/status", get(|| async { String::from("status: running") })),
                ),
        )
        .route_layer(middleware::from_fn(middlewares::logger))
        .route_layer(middleware::from_fn(middlewares::auth))
        .fallback(not_found)
}

static CONFIG: &str = include_str!("../../../config.toml");
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // init config
    // states::init_config_state(r#"config.toml"#).await;
    states::init_config_state_str(CONFIG).await;

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

    let pool = states::database::get_database_pool()?;
    if let Err(_) = commons::database::Database::migrator(pool).await {
        let cache_dir = states::config::APP_DATA.clone().join(database_location);
        if let Ok(_) = commons::util::delete_data_file(cache_dir).await {
            if let Err(_) = commons::database::Database::migrator(pool).await {
                eprintln!("database migrate failed.")
            }
        }
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5678").await?;
    let routes = build_root_router();

    axum::serve(listener, routes).await?;
    Ok(())
}
