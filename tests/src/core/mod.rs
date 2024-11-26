mod apis;
mod test_config;
mod test_db;
mod utils;

pub fn load_config() -> core::config::AppConfig {
    use core::config;
    config::AppConfig::new("..\\crates\\core\\config.toml").unwrap()
}

pub fn load_db() -> core::db::Db {
    let config = load_config();
    core::db::Db::new(config.get_cache_location()).unwrap()
}

#[test]
fn test_init_location() {
    use core::init_location;

    let config = load_config();

    init_location(config.get_locations()).unwrap();
}
