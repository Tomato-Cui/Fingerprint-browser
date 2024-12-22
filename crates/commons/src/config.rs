use config::{Config, ConfigError, FileFormat};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct App {
    pub id: String,
    pub name: String,
    pub remote_switch: bool,
    pub remote_url: String,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub driver: String,
    pub url: String,
    pub location: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Location {
    pub user_data_location: String,
    pub user_logs_location: String,
    pub browser_drivers_location: String,
    pub browser_extensions_location: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub app: App,
    pub database: Database,
}

impl AppConfig {
    pub fn build_by_filepath(config_name: &str) -> std::result::Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(config_name))
            .add_source(config::File::with_name(".").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        config.try_deserialize()
    }

    pub fn build_by_str(config_str: &str) -> std::result::Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::from_str(config_str, FileFormat::Toml))
            .add_source(config::File::with_name(".").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        config.try_deserialize()
    }

    pub fn get_database_config(&self) -> &Database {
        &self.database
    }

    pub fn get_app_config(&self) -> &App {
        &self.app
    }
}

#[test]
fn test_build_by_filepath() {
    AppConfig::build_by_filepath(r"../../config.toml").unwrap();
}

#[test]
fn test_get_database_config() {
    let config = AppConfig::build_by_filepath(r"../../config.toml").unwrap();
    let config = config.get_database_config();
    println!("{:?}", config);
}

#[test]
fn test_get_app_config() {
    let config = AppConfig::build_by_filepath(r"../../config.toml").unwrap();
    let config = config.get_app_config();
    println!("{:?}", config);
}
