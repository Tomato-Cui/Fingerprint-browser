use crate::apis::Result;
use config::{Config, ConfigError};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::sync::OnceCell;

use crate::{errors::ApplicationServerError, utils::common::app_localer};

pub static ACONFIG: OnceCell<AppConfig> = OnceCell::const_new();
pub async fn init_config(path: &str) -> &'static AppConfig {
    ACONFIG
        .get_or_init(|| async {
            let config = AppConfig::new(path).unwrap();
            config
        })
        .await
}

pub fn get_config() -> Result<&'static AppConfig> {
    match ACONFIG.get() {
        Some(c) => Ok(c),
        None => Err(ApplicationServerError::ConfigLoadError),
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::config::get_config;

    #[test]
    fn test_get_cache_location() {
        println!("{:?}", env::current_dir());
        let location = get_config().unwrap().get_cache_location();
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
    pub migrate_location: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Log {
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct Setting {
    pub data_location: String,
    pub driver_location: String,
    pub user_data_location: String,
    pub user_proxy_location: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub app: App,
    pub cache: Cache,
    pub setting: Setting,
    pub log: Log,
}

impl AppConfig {
    /// 初始化配置项
    pub fn new(config_name: &str) -> std::result::Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(config_name))
            .add_source(config::File::with_name(".").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        config.try_deserialize()
    }

    /// 获取缓存存放位置
    pub fn get_cache_location(&self) -> Result<PathBuf> {
        let locations = vec![&self.cache.location, &self.cache.name];
        Ok(app_localer::app_data_location()?.join(PathBuf::from_iter(locations)))
    }

    /// 获取用户数据存放位置
    pub fn get_user_data_location(&self) -> Result<PathBuf> {
        let locations = vec![
            &self.setting.data_location,
            &self.setting.user_data_location,
        ];
        Ok(app_localer::app_data_location()?.join(PathBuf::from_iter(locations)))
    }

    /// 获取用户代理存放位置
    pub fn get_user_proxy_location(&self) -> Result<PathBuf> {
        let locations = vec![
            &self.setting.data_location,
            &self.setting.user_proxy_location,
        ];
        Ok(app_localer::app_data_location()?.join(PathBuf::from_iter(locations)))
    }

    /// 获取浏览器驱动存放位置
    pub fn get_brower_driver_location(&self) -> Result<PathBuf> {
        let locations = vec![&self.setting.data_location, &self.setting.driver_location];
        Ok(app_localer::app_data_location()?.join(PathBuf::from_iter(locations)))
    }

    /// 获取所有相关的路径
    pub fn get_locations(&self) -> Result<Vec<PathBuf>> {
        Ok(vec![
            app_localer::app_data_location()?.join(PathBuf::from_iter(vec![&self.cache.location])),
            self.get_user_data_location()?,
            self.get_user_proxy_location()?,
            self.get_log_location()?,
            self.get_brower_driver_location()?,
        ])
    }

    /// 获取日志文件的存放位置
    pub fn get_log_location(&self) -> Result<PathBuf> {
        let locations = vec![&self.log.path];
        Ok(app_localer::app_data_location()?.join(PathBuf::from_iter(locations)))
    }
}
