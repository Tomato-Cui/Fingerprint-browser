use anyhow::anyhow;
use base64::{engine::general_purpose, DecodeError, Engine};
use std::{fs, path::PathBuf};

pub fn uuid() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string()
}

pub fn base64_encode(data: &str) -> String {
    general_purpose::STANDARD.encode(data.as_bytes())
}

pub fn base64_decode(data: &str) -> Result<Vec<u8>, DecodeError> {
    let decoded = general_purpose::STANDARD.decode(data.as_bytes())?;
    Ok(decoded)
}

pub fn md5(text: &str) -> String {
    format!("{:?}", md5::compute(text.as_bytes()))
}

pub fn generate_token(user_uuid: &str) -> Result<String, jwt::Error> {
    use aes_gcm::KeyInit;
    use hmac::Hmac;
    use jwt::SignWithKey;
    use sha2::Sha256;
    use std::collections::BTreeMap;

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("uuid", format!("{}", user_uuid));

    claims.sign_with_key(&key)
}

pub fn verify_token(token_str: &str) -> Result<String, anyhow::Error> {
    use aes_gcm::KeyInit;
    use hmac::Hmac;
    use jwt::VerifyWithKey;
    use sha2::Sha256;
    use std::collections::BTreeMap;

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret")?;
    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key)?;
    let uuid = claims.get("uuid");
    match uuid {
        Some(uuid) => return Ok(uuid.to_string()),
        None => Err(anyhow::anyhow!("token parse failed.")),
    }
}

use crate::win_imports::*;
#[cfg(windows)]
fn aes_gcm_encrypt(key: &[u8], data: &[u8]) -> std::result::Result<Vec<u8>, aes_gcm::Error> {
    pub use rand::RngCore;
    extern crate winapi;

    const ENCRYPTION_VERSION_PREFIX: &[u8] = b"v10"; // cookie前缀(Windows 和 Mac)

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
fn aes_gcm_decrypt(key: &[u8], data: &[u8]) -> std::result::Result<Vec<u8>, aes_gcm::Error> {
    pub use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Key, Nonce,
    };
    extern crate winapi;

    const ENCRYPTION_VERSION_PREFIX: &[u8] = b"v10"; // cookie前缀(Windows 和 Mac)

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
///获取加密密钥(windows 直接读取 Local State 文件)
pub async fn get_encrypt_key(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use serde_json::Value;

    let path_str: PathBuf = PathBuf::from(path).join("Local State");

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
pub fn dec_cookie(en_cookie: &[u8], key: &[u8]) -> Result<String, anyhow::Error> {
    let dec_str = aes_gcm_decrypt(&key, &en_cookie).map_err(|e| anyhow!(e.to_string()))?;
    Ok(String::from_utf8(dec_str)?)
}

#[cfg(windows)]
pub fn enc_cookie(cookie_str: &str, key: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let enc_str =
        aes_gcm_encrypt(&key, cookie_str.as_bytes()).map_err(|e| anyhow!(e.to_string()))?;
    Ok(enc_str)
}

#[cfg(not(windows))]
use crate::not_win_imports::*;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const ENCRYPTION_VERSION_PREFIX: &[u8] = b"v11"; // cookie前缀(Linux)
#[cfg(not(windows))]
type Aes128Cbc = Cbc<Aes128, Pkcs7>; // 定义 AES-128-CBC 类型
/// not window methods & variable
#[cfg(not(windows))]
/// 定义一个全局的静态变量，表示 16 个字节的空格字符
const AES_128_IV: [u8; 16] = [
    b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
];

#[cfg(not(windows))]
/// aes-128-cbc 字符串加密(linux 和 mac)
fn aes_cbc_encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
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
fn aes_cbc_decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    // 创建 AES-128-CBC 实例
    let cipher = Aes128Cbc::new_from_slices(key, &AES_128_IV)?;
    // 去掉版本前缀
    let raw_ciphertext = &data[ENCRYPTION_VERSION_PREFIX.len()..];
    let decrypted_data = cipher.decrypt_vec(&raw_ciphertext)?;
    Ok(decrypted_data)
}

#[cfg(not(windows))]
/// (非windows) 我们修改了浏览器的密钥读写方式 直接读取用户缓存目录的Breeze_key
pub fn get_encrypt_key(path: &str) -> Result<Vec<u8>> {
    use crate::config::get_config;
    use crate::public::app_file;

    let path_str: PathBuf = app_file()
        .join(get_config()?.get_user_data_location()?)
        .join(path)
        .join("Breeze_Key");
    //读取文件内容
    let file_content = fs::read_to_string(path_str)?;
    let password = base64_decode(&file_content)?;
    Ok(password)
}

#[cfg(not(windows))]
/// 解密cookie(非windows)
pub fn dec_cookie(en_cookie: &[u8], key: &[u8]) -> Result<String> {
    let dec_str = aes_cbc_decrypt(&key, &en_cookie)?;

    Ok(String::from_utf8(dec_str)?)
}

#[cfg(not(windows))]
/// 加密cookie(非windows)
pub fn enc_cookie(cookie_str: &str, key: &[u8]) -> Result<Vec<u8>> {
    let enc_str = aes_cbc_encrypt(&key, cookie_str.as_bytes())?;
    Ok(enc_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let text_md5 = md5("abc");
        println!("{}", text_md5)
    }

    #[test]
    fn test_generate_token() {
        let token = generate_token("").unwrap();
        println!("{}", token)
    }

    #[test]
    fn test_verify_token() {
        let id = verify_token(
            "eyJhbGciOiJIUzI1NiJ9.eyJpZCI6IjEifQ.WuuhtEpv0r6_AhPFzTnazBiRERZasVI5Lq7OeTogz08",
        )
        .unwrap();
        println!("{}", id)
    }
}
