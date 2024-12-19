use std::path::PathBuf;

#[allow(unused_variables)]
pub fn send_email(to: &str, from: &str, subject: &str, text: &str) {
    todo!()
}

pub fn cookie_file(path: &str) -> PathBuf {
    let path_str: PathBuf = PathBuf::from(path)
        .join("Default")
        .join("Network")
        .join("Cookies");

    path_str
}

pub async fn check_cookie_file(path: &str) -> bool {
    let path_str: PathBuf = PathBuf::from(path)
        .join("Default")
        .join("Network")
        .join("Cookies");
    path_str.exists()
}

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

#[cfg(windows)]
use std::{ffi::CString, ptr};
use winapi::um::{
    winnt::KEY_READ,
    winreg::{RegOpenKeyExA, RegQueryValueExA, HKEY_CURRENT_USER},
};
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
pub async fn get_debug_port() -> Result<u16, std::io::Error> {
    use std::net::{Ipv4Addr, SocketAddrV4};

    use tokio::net::TcpListener;

    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket).await?;
    let port = listener.local_addr()?;
    Ok(port.port())
}

pub async fn delete_data_file(path: PathBuf) -> Result<(), std::io::Error> {
    use std::fs::metadata;
    use tokio::fs::{remove_dir_all, remove_file};

    if let Ok(meta) = metadata(&path) {
        if meta.is_file() {
            remove_file(&path).await?;
        } else if meta.is_dir() {
            remove_dir_all(&path).await?;
        }
    }
    Ok(())
}
