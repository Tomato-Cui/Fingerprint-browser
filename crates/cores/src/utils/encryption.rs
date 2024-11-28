use base64::{engine::general_purpose, Engine};

use crate::apis::Result;
pub use crate::errors::ApplicationServerError;
use std::{fs, path::PathBuf};

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

/// base64 编码
pub fn base64_encode(data: &str) -> String {
    general_purpose::STANDARD.encode(data.as_bytes())
}

/// base64 解码
pub fn base64_decode(data: &str) -> Result<Vec<u8>> {
    let decoded = general_purpose::STANDARD.decode(data.as_bytes())?;
    Ok(decoded)
}

/// not window methods & variable
#[cfg(not(windows))]
/// 定义一个全局的静态变量，表示 16 个字节的空格字符
const AES_128_IV: [u8; 16] = [
    b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
];

#[cfg(not(windows))]
/// 返回用户cookie文件(非windows系统)
pub fn cookie_file(path: &str) -> PathBuf {
    let path_str: PathBuf = app_file()
        .join(get_config()?.get_user_data_location())
        .join(path)
        .join("Default")
        .join("Cookies");
    path_str
}

#[cfg(not(windows))]
/// 检查cookie文件是否存在
pub fn check_cookie_file(path: &str) -> bool {
    let path_str: PathBuf = app_file()
        .join(get_config()?.get_user_data_location())
        .join(path)
        .join("Default")
        .join("Cookies");
    path_str.exists()
}

#[cfg(not(windows))]
/// aes-128-cbc 字符串加密(linux 和 mac)
fn aes_cbc_encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ApplicationServerError> {
    let cipher = Aes128Cbc::new_from_slices(key, &AES_128_IV)?; // 创建 AES-128-CBC 实例
    let ciphertext = cipher.encrypt_vec(data); // 加密明文
                                               // 在密文前面插入版本前缀
    let mut result = Vec::with_capacity(ENCRYPTION_VERSION_PREFIX.len() + ciphertext.len());
    result.extend_from_slice(ENCRYPTION_VERSION_PREFIX);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

#[cfg(not(windows))]
/// aes-128-cbc 字符串解密(linux 和 mac)
fn aes_cbc_decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ApplicationServerError> {
    let cipher = Aes128Cbc::new_from_slices(key, &AES_128_IV)?; // 创建 AES-128-CBC 实例
                                                                // 去掉版本前缀
    let raw_ciphertext = &data[ENCRYPTION_VERSION_PREFIX.len()..];
    // 解密
    let decrypted_data = cipher.decrypt_vec(&raw_ciphertext)?;
    Ok(decrypted_data)
}

#[cfg(not(windows))]
/// (非windows) 我们修改了浏览器的密钥读写方式 直接读取用户缓存目录的Breeze_key
pub fn get_encrypt_key(path: &str) -> Result<Vec<u8>, ApplicationServerError> {
    let path_str: PathBuf = app_file()
        .join(get_config()?.get_user_data_location())
        .join(path)
        .join("Breeze_Key");
    //读取文件内容
    let file_content = fs::read_to_string(path_str)?;
    let password = base64_decode(&file_content)?;
    Ok(password)
}

#[cfg(not(windows))]
/// 解密cookie(非windows)
pub fn dec_cookie(en_cookie: &[u8], key: &[u8]) -> Result<String, ApplicationServerError> {
    let dec_str = aes_cbc_decrypt(&key, &en_cookie)?;

    Ok(String::from_utf8(dec_str)?)
}

#[cfg(not(windows))]
/// 加密cookie(非windows)
pub fn enc_cookie(cookie_str: &str, key: &[u8]) -> Result<Vec<u8>, ApplicationServerError> {
    let enc_str = aes_cbc_encrypt(&key, cookie_str.as_bytes())?;
    Ok(enc_str)
}

/// window methods
#[cfg(windows)]
/// aes-gcm 加密字符串(windows)
fn aes_gcm_encrypt(key: &[u8], data: &[u8]) -> std::result::Result<Vec<u8>, aes_gcm::Error> {
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
///aes-gcm 解密字符串
fn aes_gcm_decrypt(key: &[u8], data: &[u8]) -> std::result::Result<Vec<u8>, aes_gcm::Error> {
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
///返回用户cookie文件(windows系统)
pub fn cookie_file(path: &str) -> Result<PathBuf> {
    use super::common::app_localer;
    use crate::config::get_config;

    let path_str: PathBuf = app_localer::app_data_location()?
        .join(get_config()?.get_user_data_location()?)
        .join(path)
        .join("Default")
        .join("Network")
        .join("Cookies");
    Ok(path_str)
}

#[cfg(windows)]
/// 检查cookie是否存在  
pub fn check_cookie_file(path: &str) -> Result<bool> {
    use super::common::app_localer;
    use crate::config::get_config;

    let path_str: PathBuf = app_localer::app_data_location()?
        .join(get_config()?.get_user_data_location()?)
        .join(path)
        .join("Default")
        .join("Network")
        .join("Cookies");
    Ok(path_str.exists())
}

#[cfg(windows)]
///获取加密密钥(windows 直接读取 Local State 文件)
pub fn get_encrypt_key(path: &str) -> Result<Vec<u8>> {
    use crate::config::get_config;

    use super::common::app_localer;

    let path_str: PathBuf = app_localer::app_data_location()?
        .join(get_config()?.get_user_data_location()?)
        .join(path)
        .join("Local State");
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
///调用系统win api
fn decrypt_string_with_dpapi(encrypted_data: &[u8]) -> std::result::Result<Vec<u8>, DWORD> {
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
///解密cookie(windows)
pub fn dec_cookie(en_cookie: &[u8], key: &[u8]) -> Result<String> {
    let dec_str = aes_gcm_decrypt(&key, &en_cookie).unwrap();
    Ok(String::from_utf8(dec_str)?)
}

#[cfg(windows)]
///加密cookie(windows)
pub fn enc_cookie(cookie_str: &str, key: &[u8]) -> Result<Vec<u8>> {
    let enc_str = aes_gcm_encrypt(&key, cookie_str.as_bytes()).unwrap();
    Ok(enc_str)
}
