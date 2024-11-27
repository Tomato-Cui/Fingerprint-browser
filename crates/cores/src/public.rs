use base64::{engine::general_purpose, Engine};
use chrono::Utc;

pub use crate::errors::ApplicationServerError;
use std::fs::{metadata, remove_dir_all, remove_file};
use std::process::{Child, Command, Stdio};
use std::{env, fs, fs::canonicalize, path::PathBuf};
use tauri::api::path::local_data_dir;

#[cfg(not(windows))]
use crate::not_win_imports::*;

#[cfg(windows)]
use crate::win_imports::*;

#[cfg(any(target_os = "windows", target_os = "macos"))]
const ENCRYPTION_VERSION_PREFIX: &[u8] = b"v10"; // cookie前缀(Windows 和 Mac)
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const ENCRYPTION_VERSION_PREFIX: &[u8] = b"v11"; // cookie前缀(Linux)
#[cfg(not(windows))]
type Aes128Cbc = Cbc<Aes128, Pkcs7>; // 定义 AES-128-CBC 类型

#[cfg(not(windows))]
// 定义一个全局的静态变量，表示 16 个字节的空格字符
const AES_128_IV: [u8; 16] = [
    b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
];
const APP_ID: &str = "com.breeze.browser"; //应用包名
const BREEZE_DATABASE: &str = "breeze_db"; //程序数据库

/////////////////////////////////////////////公共方法
//返回软件数据目录
pub fn app_file() -> PathBuf {
    local_data_dir()
        .expect("failed to resolve home_dir")
        .join(format!("{}", &APP_ID))
}

pub fn timestamp_to_seconds(timestamp_microseconds: u64) -> f64 {
    let timestamp_seconds = timestamp_microseconds as f64 / 1_000_000.0; // Convert to seconds
    let windows_to_unix_epoch_diff = 11_644_473_600.0; // Difference between Unix and Windows epochs
    let unix_timestamp_seconds = timestamp_seconds - windows_to_unix_epoch_diff; // Convert to Unix timestamp
    unix_timestamp_seconds
}

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

pub fn bool_to_int(value: bool) -> i32 {
    if value {
        1
    } else {
        0
    }
}

//返回程序数据库文件
pub fn breeze_db_file() -> PathBuf {
    app_file().join(BREEZE_DATABASE)
}

//返回软件根目录
pub fn app_root() -> PathBuf {
    // 获取当前程序的完整路径
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    // 获取程序的根目录
    let root_dir =
        canonicalize(exe_path.parent().unwrap()).expect("Failed to get parent directory");
    root_dir
}

//初始化配置文件目录 先判断目录是否存在，不存在则创建
pub fn init_settings() -> Result<(), ApplicationServerError> {
    let user_data: PathBuf = app_file().join("user_data");
    if !user_data.exists() {
        std::fs::create_dir_all(&user_data)?;
    }
    let user_porxy: PathBuf = app_file().join("user_porxy");
    if !user_porxy.exists() {
        std::fs::create_dir_all(&user_porxy)?;
    }
    Ok(())
}

//删除指定文件
pub fn delete_file(base_path: &str, path: &str) -> Result<(), ApplicationServerError> {
    let path_buf = app_file().join(base_path).join(path);

    if let Ok(meta) = metadata(&path_buf) {
        if meta.is_file() {
            // 如果是文件，则删除文件
            remove_file(&path_buf)?;
        } else if meta.is_dir() {
            // 如果是目录，则删除目录及其内容
            remove_dir_all(&path_buf)?;
        }
    }
    Ok(())
}

//启动浏览器
pub fn start_browser(
    ua: &str,
    user_data: &str,
    proxy: &str,
    fp_info: &str,
    lang: &str,
) -> Result<Child, ApplicationServerError> {
    let mut args: Vec<String> = vec![
        "--no-default-browser-check".into(),
        "--no-first-run".into(),
        format!("--accept-lang={}", lang),
        format!("--user-agent={}", ua),
        format!(
            "--user-data-dir={}",
            app_file()
                .join("user_data")
                .join(user_data)
                .to_str()
                .unwrap()
        ),
        format!("--breeze-fp={}", base64_encode(fp_info)),
    ];
    if proxy != "null" {
        args.push(format!("--proxy-server={}", proxy));
    }

    // let browser_path = app_root().join("BreezeBrowser").join("chrome");
    let browser_path = app_root()
        .join("BreezeBrowser")
        .join("C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe ");

    let t = browser_path.to_str();
    println!("{:?}", t);

    let child = Command::new(browser_path)
        .args(args)
        .stderr(Stdio::null()) // 重定向标准错误输出
        .stdout(Stdio::null()) // 重定向标准输出
        .spawn()
        .unwrap();

    Ok(child)
}

//base64 编码
fn base64_encode(data: &str) -> String {
    general_purpose::STANDARD.encode(data.as_bytes())
}

//base64 解码
fn base64_decode(data: &str) -> Result<Vec<u8>, ApplicationServerError> {
    let decoded = general_purpose::STANDARD.decode(data.as_bytes())?;
    Ok(decoded)
}
/////////////////////////////////////////////非windows系统 方法
#[cfg(not(windows))]
//返回用户cookie文件(非windows系统)
pub fn cookie_file(path: &str) -> PathBuf {
    let path_str: PathBuf = app_file()
        .join("user_data")
        .join(path)
        .join("Default")
        .join("Cookies");
    path_str
}
#[cfg(not(windows))]
pub fn check_cookie_file(path: &str) -> bool {
    let path_str: PathBuf = app_file()
        .join("user_data")
        .join(path)
        .join("Default")
        .join("Cookies");
    path_str.exists()
}

//aes-128-cbc 字符串加密(linux 和 mac)
#[cfg(not(windows))]
fn aes_cbc_encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ApplicationServerError> {
    let cipher = Aes128Cbc::new_from_slices(key, &AES_128_IV)?; // 创建 AES-128-CBC 实例
    let ciphertext = cipher.encrypt_vec(data); // 加密明文
                                               // 在密文前面插入版本前缀
    let mut result = Vec::with_capacity(ENCRYPTION_VERSION_PREFIX.len() + ciphertext.len());
    result.extend_from_slice(ENCRYPTION_VERSION_PREFIX);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

//aes-128-cbc 字符串解密(linux 和 mac)
#[cfg(not(windows))]
fn aes_cbc_decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ApplicationServerError> {
    let cipher = Aes128Cbc::new_from_slices(key, &AES_128_IV)?; // 创建 AES-128-CBC 实例
                                                                // 去掉版本前缀
    let raw_ciphertext = &data[ENCRYPTION_VERSION_PREFIX.len()..];
    // 解密
    let decrypted_data = cipher.decrypt_vec(&raw_ciphertext)?;
    Ok(decrypted_data)
}

#[cfg(not(windows))]
//(非windows) 我们修改了浏览器的密钥读写方式 直接读取用户缓存目录的Breeze_key
pub fn get_encrypt_key(path: &str) -> Result<Vec<u8>, ApplicationServerError> {
    let path_str: PathBuf = app_file().join("user_data").join(path).join("Breeze_Key");
    //读取文件内容
    let file_content = fs::read_to_string(path_str)?;
    let password = base64_decode(&file_content)?;
    Ok(password)
}

#[cfg(not(windows))]
//解密cookie(非windows)
pub fn dec_cookie(en_cookie: &[u8], key: &[u8]) -> Result<String, ApplicationServerError> {
    let dec_str = aes_cbc_decrypt(&key, &en_cookie)?;

    Ok(String::from_utf8(dec_str)?)
}

#[cfg(not(windows))]
//加密cookie(非windows)
pub fn enc_cookie(cookie_str: &str, key: &[u8]) -> Result<Vec<u8>, ApplicationServerError> {
    let enc_str = aes_cbc_encrypt(&key, cookie_str.as_bytes())?;
    Ok(enc_str)
}
//////////////////////////////////windows系统 方法
#[cfg(windows)]
// aes-gcm 加密字符串(windows)
fn aes_gcm_encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let keys: &Key<Aes256Gcm> = key.into();
    // 创建 AES-256-GCM 实例
    let cipher = Aes256Gcm::new(keys);
    // 生成随机 nonce
    let mut nonce_bytes = vec![0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes); // 创建 nonce

    let mut ciphertext = cipher.encrypt(nonce, data)?;
    // 将 nonce 和版本前缀插入到密文中
    ciphertext.splice(0..0, nonce_bytes); // 在开头插入 nonce
    ciphertext.splice(0..0, ENCRYPTION_VERSION_PREFIX.to_vec()); // 在开头插入版本前缀

    Ok(ciphertext)
}

#[cfg(windows)]
//aes-gcm 解密字符串
fn aes_gcm_decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let keys: &Key<Aes256Gcm> = key.into();
    // 创建 AES-256-GCM 实例
    let cipher = Aes256Gcm::new(keys);
    // 提取 nonce
    let nonce_start = ENCRYPTION_VERSION_PREFIX.len();
    let nonce_end = nonce_start + 12;
    let nonce_bytes = &data[nonce_start..nonce_end];
    let nonce = Nonce::from_slice(nonce_bytes); // 创建 nonce
    let raw_ciphertext = &data[nonce_end..]; // 提取原始密文
                                             // 解密
    let plaintext = cipher.decrypt(nonce, raw_ciphertext)?;
    Ok(plaintext)
}

#[cfg(windows)]
//返回用户cookie文件(windows系统)
pub fn cookie_file(path: &str) -> PathBuf {
    let path_str: PathBuf = app_file()
        .join("user_data")
        .join(path)
        .join("Default")
        .join("Network")
        .join("Cookies");
    path_str
}

#[cfg(windows)]
pub fn check_cookie_file(path: &str) -> bool {
    let path_str: PathBuf = app_file()
        .join("user_data")
        .join(path)
        .join("Default")
        .join("Network")
        .join("Cookies");
    path_str.exists()
}

#[cfg(windows)]
//获取加密密钥(windows 直接读取 Local State 文件)
pub fn get_encrypt_key(path: &str) -> Result<Vec<u8>, ApplicationServerError> {
    let path_str: PathBuf = app_file().join("user_data").join(path).join("Local State");
    //读取文件内容
    let file_content = fs::read_to_string(path_str)?;
    //将JSON字符串转换为Value对象
    let json_data: Value = serde_json::from_str(&file_content)?;
    // 提取 os_crypt.encrypted_key 的值
    if let Some(encrypted_key) = json_data
        .get("os_crypt")
        .and_then(|os_crypt| os_crypt.get("encrypted_key"))
    {
        //base64解码
        let decoded_key = base64_decode(encrypted_key.as_str().unwrap())?;
        let k_dpapikey_prefix = "DPAPI";
        let prefix_length = k_dpapikey_prefix.len();
        let encrypted_key = &decoded_key[prefix_length..];
        let encrypted_key_dpapi = decrypt_string_with_dpapi(encrypted_key).unwrap();

        //返回解密后的密钥
        return Ok(encrypted_key_dpapi);
    } else {
        return Ok(vec![]);
    }
}

#[cfg(windows)]
//调用系统win api
fn decrypt_string_with_dpapi(encrypted_data: &[u8]) -> Result<Vec<u8>, DWORD> {
    unsafe {
        let mut in_blob = DATA_BLOB {
            cbData: encrypted_data.len() as DWORD,
            pbData: encrypted_data.as_ptr() as *mut _,
        };
        let mut out_blob = DATA_BLOB {
            cbData: 0,
            pbData: ptr::null_mut(),
        };
        let result = CryptUnprotectData(
            &mut in_blob,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            &mut out_blob,
        );

        if result == 1 {
            let decrypted_data = Vec::from_raw_parts(
                out_blob.pbData,
                out_blob.cbData as usize,
                out_blob.cbData as usize,
            );
            Ok(decrypted_data)
        } else {
            Err(result.try_into().unwrap())
        }
    }
}

#[cfg(windows)]
//解密cookie(windows)
pub fn dec_cookie(en_cookie: &[u8], key: &[u8]) -> Result<String, ApplicationServerError> {
    let dec_str = aes_gcm_decrypt(&key, &en_cookie).unwrap();
    Ok(String::from_utf8(dec_str)?)
}

#[cfg(windows)]
//加密cookie(windows)
pub fn enc_cookie(cookie_str: &str, key: &[u8]) -> Result<Vec<u8>, ApplicationServerError> {
    let enc_str = aes_gcm_encrypt(&key, cookie_str.as_bytes()).unwrap();
    Ok(enc_str)
}
