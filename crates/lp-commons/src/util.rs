use std::{
    fs,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

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

use serde::Serialize;
use serde_json::Value;
#[cfg(windows)]
use std::{ffi::CString, ptr};
use tokio::net::TcpStream;
use winapi::um::{
    winnt::KEY_READ,
    winreg::{RegOpenKeyExA, RegQueryValueExA, HKEY_CURRENT_USER},
};

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
    let port = listener.local_addr()?.port();

    drop(listener);

    Ok(port)
}

pub async fn get_network_ip() -> Result<IpAddr, anyhow::Error> {
    let remote_addr: SocketAddr = "8.8.8.8:80".parse()?;
    let stream = TcpStream::connect(remote_addr).await?;
    let local_addr = stream.local_addr()?;

    Ok(local_addr.ip())
}

pub fn delete_data_file(path: PathBuf) -> Result<(), anyhow::Error> {
    use std::fs;

    let abc = fs::remove_dir_all(&path)
        .map_err(|v| anyhow::anyhow!("remove current path target failed: {:?}", v));
    eprintln!("{:?}", abc);

    Ok(())
}

pub fn struct_to_json_value(value: impl Serialize) -> Value {
    match serde_json::to_value(value) {
        Ok(v) => v,
        Err(_) => serde_json::to_value("").unwrap_or_default(),
    }
}

pub fn send_email(
    smtp_username: &str,
    smtp_password: &str,
    smtp_server: &str,
    to: &str,
    title: &str,
    body: &str,
) -> Result<bool, anyhow::Error> {
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    let email = Message::builder()
        .from(
            smtp_username
                .parse()
                .map_err(|_| anyhow::anyhow!("from email is failed."))?,
        )
        .to(to
            .parse()
            .map_err(|_| anyhow::anyhow!("receive email is failed."))?)
        .subject(title)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))?;
    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(true),
        Err(_e) => Err(anyhow::anyhow!("email send failed.")),
    }
}

pub fn calculate_pagination(page_num: u32, page_size: u32, total: i64) -> (u32, u32) {
    let page_num = if page_num <= 0 {
        1
    } else if (page_num * page_size) as i64 > total {
        (total as f64 / page_size as f64).ceil() as u32
    } else {
        page_num
    };
    let mut offset = (page_num - 1) * page_size;
    offset = if offset <= 0 { 0 } else { offset };
    (offset, page_size)
}

pub fn unzip(filepath: &str, extract_to: &str) -> Result<bool, anyhow::Error> {
    let file = fs::File::open(filepath)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let extract_path = std::path::Path::new(extract_to);
    let extract_path = extract_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("extract path load failed."))?;
    if !extract_path.exists() {
        fs::create_dir_all(extract_path)?;
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => extract_path.join(path),
            None => continue,
        };

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(true)
}
