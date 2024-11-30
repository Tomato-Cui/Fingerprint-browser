use std::{
    fmt::Debug,
    net::{Ipv4Addr, SocketAddrV4},
};

use serde::Serialize;
use tokio::net::TcpListener;

use super::fs::ApplicationServerError;

#[cfg(windows)]
use std::{ffi::CString, ptr};

pub mod app_localer {
    use crate::config;
    use crate::Result;
    use std::{env, fs::canonicalize, path::PathBuf};
    use tauri::api::path::local_data_dir;

    ///返回软件数据目录
    /// examples: "C:\\Users\\cgy\\AppData\\Local\\com.ads-hubstudio.browser"
    pub fn app_data_location() -> Result<PathBuf> {
        Ok(local_data_dir()
            .expect("failed to resolve home_dir")
            .join(format!("{}", &config::get_config()?.app.id)))
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

/// option<vec<string>> to string
pub fn option_vec_string_to_string(values: Option<Vec<String>>, sep: &str) -> String {
    match values {
        Some(v) => v.join(sep),
        None => "".to_string(),
    }
}

/// option<vec<string>> <- string
pub fn string_to_option_vec_string(values: Option<String>, sep: &str) -> Option<Vec<String>> {
    match values {
        Some(v) => Some(v.split(sep).map(|v| v.to_string()).collect()),
        None => Some(Vec::new()),
    }
}

/// 将结构体转换为String
pub fn to_string<T>(value: T) -> Result<String, ApplicationServerError>
where
    T: Serialize + Debug,
{
    Ok(serde_json::to_string(&value)?)
}

#[cfg(windows)]
use winapi::um::{
    winnt::KEY_READ,
    winreg::{RegOpenKeyExA, RegQueryValueExA, HKEY_CURRENT_USER},
};

#[cfg(not(windows))]
pub fn get_proxy_from_registry() -> Option<String> {
    use std::env;
    match env::var("HTTP_PROXY")
        .or_else(|_| env::var("HTTPS_PROXY"))
        .or_else(|_| env::var("ALL_PROXY"))
    {
        Ok(v) => Some(v),
        _ => None,
    }
}

/// TODO: 未做其他系统的兼容。
/// 获取系统代理
#[cfg(windows)]
pub fn get_proxy_from_registry() -> Option<String> {
    let key = CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings");
    let key = match key {
        Ok(k) => k,
        Err(_) => return None,
    };

    let mut hkey = ptr::null_mut();
    unsafe {
        if RegOpenKeyExA(HKEY_CURRENT_USER, key.as_ptr(), 0, KEY_READ, &mut hkey) == 0 {
            let mut value = vec![0u8; 1024];
            let mut value_len = value.len() as u32;
            let proxy_value_name = CString::new("ProxyServer").unwrap();

            if RegQueryValueExA(
                hkey,
                proxy_value_name.as_ptr(),
                0 as *mut u32,
                ptr::null_mut(),
                value.as_mut_ptr(),
                &mut value_len,
            ) == 0
            {
                let proxy_str = String::from_utf8_lossy(&value[..value_len as usize])
                    .trim_end_matches('\0') // Remove trailing null bytes
                    .to_string();
                return Some(proxy_str);
            }
        }
    }

    None
}

#[cfg(not(windows))]
pub fn get_chrome_install_path() -> Option<String> {
    None
}

#[cfg(windows)]
pub fn get_chrome_install_path() -> Option<String> {
    use winapi::um::winreg::HKEY_CLASSES_ROOT;
    let key_path = CString::new("ChromeHTML\\shell\\open\\command").unwrap();
    let mut hkey = ptr::null_mut();

    unsafe {
        if RegOpenKeyExA(HKEY_CLASSES_ROOT, key_path.as_ptr(), 0, KEY_READ, &mut hkey) == 0 {
            let mut value = vec![0u8; 1024];
            let mut value_len = value.len() as u32;
            let value_name = CString::new("").unwrap();

            if RegQueryValueExA(
                hkey,
                value_name.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                value.as_mut_ptr(),
                &mut value_len,
            ) == 0
            {
                let command_str = String::from_utf8_lossy(&value[..value_len as usize]);

                if let Some(first_quote_pos) = command_str.find('"') {
                    if let Some(second_quote_pos) = command_str[first_quote_pos + 1..].find('"') {
                        let chrome_path = &command_str
                            [first_quote_pos + 1..first_quote_pos + second_quote_pos + 1];
                        return Some(chrome_path.to_string());
                    }
                }
            }
        }
    }

    None
}

/// 获取随机的一个端口
pub async fn get_debug_port() -> Result<u16, ApplicationServerError> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket).await?;
    let port = listener.local_addr()?;
    // println!("Listening on {}, access this port to end the program", port);
    // let (mut tcp_stream, addr) = listener.accept()?; // 阻塞，直到被请求
    // println!("Connection received! {:?} is sending data.", addr);
    // let mut input = String::new();
    // let _ = tcp_stream.read_to_string(&mut input)?;
    // println!("{:?} says {}", addr, input);
    Ok(port.port())
}
