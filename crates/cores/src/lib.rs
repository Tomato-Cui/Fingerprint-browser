use std::path::PathBuf;

use errors::ApplicationServerError;
use state::get_app_cache_location;

pub mod processor;

pub mod apis;
pub mod auth;
pub mod config;
pub mod database;
pub mod errors;
pub mod models;
pub mod requests;
pub mod state;
pub mod utils;

#[cfg(not(windows))]
mod not_win_imports {
    pub use aes::Aes128;
    pub use block_modes::{BlockMode, Cbc};
    pub use block_padding::Pkcs7;
}

#[cfg(target_os = "windows")]
mod win_imports {
    pub use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Key, Nonce,
    };
    pub use rand::RngCore;
    pub use serde_json::Value;
    extern crate winapi;

    pub use std::ptr;
    pub use winapi::shared::minwindef::DWORD;
    pub use winapi::um::dpapi::CryptUnprotectData;
    pub use winapi::um::wincrypt::DATA_BLOB;
}

/// 初始化应用的配置文件目录
///
/// 先判断是否存在，不存在就创建
pub async fn init_location(locations: Vec<PathBuf>) -> Result<()> {
    for location in locations {
        let setting_file_location = get_app_cache_location().await?.join(location);

        if !setting_file_location.exists() {
            std::fs::create_dir_all(setting_file_location)?;
        }
    }

    Ok(())
}

pub type Result<T> = core::result::Result<T, ApplicationServerError>;
