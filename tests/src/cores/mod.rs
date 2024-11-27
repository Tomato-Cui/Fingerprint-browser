mod apis;
mod test_config;
mod test_db;
mod utils;

pub fn load_config() -> cores::config::AppConfig {
    use cores::config;
    config::AppConfig::new(r#"..\config.toml"#).unwrap()
}

pub fn load_db() -> cores::db::Db {
    let config = load_config();
    cores::db::Db::new(config.get_cache_location()).unwrap()
}

#[test]
fn test_init_location() {
    use cores::init_location;

    let config = load_config();

    init_location(config.get_locations()).unwrap();
}
