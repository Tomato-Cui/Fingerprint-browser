use std::path::PathBuf;

use errors::ApplicationServerError;
use utils::common::app_localer;

pub mod apis;
pub mod config;
pub mod db;
pub mod errors;
pub mod models;
pub mod public;
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
pub fn init_location(locations: Vec<PathBuf>) -> Result<()> {
    locations.iter().try_for_each(|v| {
        let setting_file_location = app_localer::app_data_location()?.join(&v);

        if !setting_file_location.exists() {
            std::fs::create_dir_all(setting_file_location)?;
        }

        Ok::<(), ApplicationServerError>(())
    })?;

    Ok(())
}

pub type Result<T> = core::result::Result<T, ApplicationServerError>;
