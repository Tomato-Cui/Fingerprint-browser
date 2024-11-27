use config::{Config, ConfigError};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::path::PathBuf;

use crate::utils::common::app_localer;

lazy_static! {
    // pub static ref AConfig: AppConfig = AppConfig::new("config.toml").unwrap();
    pub static ref AConfig: AppConfig = AppConfig::new(r#"..\config.toml"#).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_cache_location() {
        println!("{:?}", env::current_dir());
        let location = AConfig.get_cache_location();

        println!("{:?}", location)
    }
}

#[derive(Deserialize, Debug)]
pub struct App {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Cache {
    pub name: String,
    pub location: String,
}

#[derive(Deserialize, Debug)]
pub struct Setting {
    pub data_location: String,
    pub user_data_location: String,
    pub user_proxy_location: String,
    pub all_locations: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub app: App,
    pub cache: Cache,
    pub setting: Setting,
}

impl AppConfig {
    /// 初始化配置项
    pub fn new(config_name: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(config_name))
            .add_source(config::File::with_name(".").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        config.try_deserialize()
    }

    /// 获取缓存存放位置
    pub fn get_cache_location(&self) -> PathBuf {
        let locations = vec![&self.cache.location, &self.cache.name];
        app_localer::app_data_location().join(PathBuf::from_iter(locations))
    }

    /// 获取用户数据存放位置
    pub fn get_user_data_location(&self) -> PathBuf {
        let locations = vec![
            &self.setting.data_location,
            &self.setting.user_data_location,
        ];
        app_localer::app_data_location().join(PathBuf::from_iter(locations))
    }

    /// 获取用户代理存放位
    pub fn get_user_proxy_location(&self) -> PathBuf {
        let locations = vec![
            &self.setting.data_location,
            &self.setting.user_proxy_location,
        ];
        app_localer::app_data_location().join(PathBuf::from_iter(locations))
    }

    /// 获取所有相关的路径
    pub fn get_locations(&self) -> Vec<PathBuf> {
        vec![
            app_localer::app_data_location().join(PathBuf::from_iter(vec![&self.cache.location])),
            self.get_user_data_location(),
            self.get_user_proxy_location(),
        ]
    }
}
