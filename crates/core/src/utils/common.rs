use std::fmt::Debug;

use serde::Serialize;

use super::fs::ApplicationServerError;

pub mod app_localer {
    use crate::config;
    use std::{env, fs::canonicalize, path::PathBuf};
    use tauri::api::path::local_data_dir;

    ///返回软件数据目录
    /// examples: "C:\\Users\\cgy\\AppData\\Local\\com.ads-hubstudio.browser"
    pub fn app_data_location() -> PathBuf {
        local_data_dir()
            .expect("failed to resolve home_dir")
            .join(format!("{}", &config::AConfig.app.id))
    }

    ///返回软件目录
    pub fn app_location() -> PathBuf {
        // 获取当前程序的完整路径
        let exe_path = env::current_exe().expect("Failed to get current executable path");
        // 获取程序的根目录
        let root_dir =
            canonicalize(exe_path.parent().unwrap()).expect("Failed to get parent directory");
        root_dir
    }
}

pub mod app_timer {
    use chrono::Utc;

    /// 时间戳转换成时间
    pub fn timestamp_to_seconds(timestamp_microseconds: u64) -> f64 {
        let timestamp_seconds = timestamp_microseconds as f64 / 1_000_000.0; // Convert to seconds
        let windows_to_unix_epoch_diff = 11_644_473_600.0; // Difference between Unix and Windows epochs
        let unix_timestamp_seconds = timestamp_seconds - windows_to_unix_epoch_diff; // Convert to Unix timestamp
        unix_timestamp_seconds
    }

    /// 时间转换成时间戳
    pub fn seconds_to_timestamp(unix_timestamp_seconds: f64) -> u64 {
        let windows_to_unix_epoch_diff = 11_644_473_600.0; // Difference between Unix and Windows epochs
        let timestamp_seconds = unix_timestamp_seconds + windows_to_unix_epoch_diff; // Convert to Windows timestamp
        let timestamp_microseconds = (timestamp_seconds * 1_000_000.0) as u64; // Convert to microseconds
        timestamp_microseconds
    }

    /// 返回从 Unix 时间开始以来的纳秒数。
    pub fn generate_nanosecond_timestamp() -> u64 {
        let now = Utc::now(); // Get the current time
        let unix_epoch: u64 = now.timestamp().try_into().unwrap(); // Convert to Unix timestamp (seconds)
        let windows_to_unix_epoch_diff = 11_644_473_600; // Difference between Unix and Windows epochs
        let windows_epoch = unix_epoch + windows_to_unix_epoch_diff; // Convert to Windows timestamp (seconds)
        let windows_epoch_microseconds = windows_epoch * 1_000_000; // Convert to microseconds
        windows_epoch_microseconds
    }
}

/// bool to int
pub fn bool_to_int(value: bool) -> i32 {
    if value {
        1
    } else {
        0
    }
}

/// 将结构体转换为String
pub fn to_string<T>(value: T) -> Result<String, ApplicationServerError>
where
    T: Serialize + Debug,
{
    Ok(serde_json::to_string(&value)?)
}
