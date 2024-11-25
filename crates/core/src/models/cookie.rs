/// cookie data models
use crate::{
    db::Db,
    errors::ApplicationServerError,
    utils::{
        common::{
            app_timer::{
                generate_nanosecond_timestamp, seconds_to_timestamp, timestamp_to_seconds,
            },
            bool_to_int,
        },
        encryption::{cookie_file, dec_cookie, enc_cookie, get_encrypt_key},
    },
};
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// 定义cookie数据结构(浏览器插件 例如: Cookie Editor)
///
///# 数据库表和插件的值 对应关系
///- db[1]: host_key 对应 pugins: domain des: Cookie所在的网页(domain)
///- db[3]: name 对应 pugins: name des: Cookie的名称(name)
///- db[5]：encrypted_value 对应 pugins: value des: Cookie的值(value) 需要加解密
///- db[6]: path 对应 pugins: path des: Cookie所在的路径(path)
///- db[7]: expires_utc 对应 pugins: expiration_date des: Cookie的有效期限(expiration_date) 注意 需要转换数据类型
///- db[8]: is_secure 对应 pugins: secure des: 指示在浏览器与服务器之间传输该Cookie时需要采用加密通道，即https
///- db[9]: is_httponly 对应 pugins: http_only des: 指示Cookie是否只能通过HTTP协议传输 防止js读取Cookie
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginsCookie {
    pub creation_utc: String, // 必填  由于数据库没有主键  经过验证  好像只有创建时间是唯一不重复的值  那就用这个当主键 (这个大数在前端反序列化时会丢失精度  所以定义类型为： String 需要时转为 u64)
    pub domain: String,       // 必填
    #[serde(rename = "expirationDate")]
    pub expiration_date: String, // 可选的 这个索性也用 String 类型吧
    //#[serde(rename = "hostOnly")]
    //pub host_only: Option<bool>,      // 可选的 指示domain是否合法  false: 域名不合法 例如 .xxx.com true: 合法 例如 www.xxx.com
    #[serde(rename = "httpOnly")]
    pub http_only: bool, // 必填
    pub name: String, // 必填
    pub path: String, // 必填
    //#[serde(rename = "sameSite")]
    //pub same_site: Option<String>,  // 可选的 Option<String> 以支持 null sameSite的值如果为None secure必须为true
    pub secure: bool, // 必填
    //pub session: bool,              // 可选的
    //pub store_id: Option<String>,   // 可选的
    pub value: String, // 必填
}

impl PluginsCookie {
    /// cookie表中查询所有数据
    pub fn query_cookie(path: &str) -> Result<String, ApplicationServerError> {
        let key = get_encrypt_key(path)?;

        let db = Db::new(cookie_file(path))?;

        let sql = "select creation_utc, host_key, name, encrypted_value, path, expires_utc, is_secure, is_httponly from cookies";
        let mut stmt = db.query_map_table(sql)?;

        let cookie_iter = stmt.query_map(params![], |row| {
            let expiration = row.get::<_, u64>(5)?;
            let encrypted_value = row.get::<_, Vec<u8>>(3)?;
            let dec_val: String;
            if encrypted_value.len() > 0 {
                dec_val = dec_cookie(&encrypted_value, &key).unwrap();
            } else {
                dec_val = String::new();
            }
            let unix_timestamp_seconds = timestamp_to_seconds(expiration);
            let creation_time = row.get::<_, u64>(0)?;
            Ok(PluginsCookie {
                creation_utc: creation_time.to_string(),
                domain: row.get(1)?,
                expiration_date: unix_timestamp_seconds.to_string(),
                http_only: row.get(7)?,
                name: row.get(2)?,
                path: row.get(4)?,
                secure: row.get(6)?,
                value: dec_val,
            })
        })?;
        //db 数据库会在离开作用域后自动断开
        let cookie: Vec<PluginsCookie> = cookie_iter.filter_map(Result::ok).collect();

        Ok(serde_json::to_string(&cookie)?)
    }

    /// cookie表中更新数据
    pub fn update_cookie(path: &str, cookie: &str) -> Result<bool, ApplicationServerError> {
        let key = get_encrypt_key(path)?;
        let db = Db::new(cookie_file(path))?;
        let cookies: PluginsCookie = serde_json::from_str(cookie)?;

        let en_cookie_val: Vec<u8>;

        if cookies.value.is_empty() {
            en_cookie_val = Vec::new(); // 如果 cookies.value 为空，插入一个空的 Vec<u8>
        } else {
            en_cookie_val = enc_cookie(&cookies.value, &key)?; // 如果 cookies.value 不为空，插入加密后的值
        }
        //根据creation_utc 更新cookie
        let sql = "
        UPDATE cookies
        SET host_key = ?1,
            encrypted_value = ?2,
            path = ?3,
            expires_utc = ?4,
            is_secure = ?5,
            is_httponly = ?6,
            name = ?7
        WHERE
            creation_utc = ?8";
        let status = db.query_table(
            &sql,
            params![
                cookies.domain,
                &en_cookie_val,
                cookies.path,
                seconds_to_timestamp(cookies.expiration_date.parse::<f64>().unwrap()),
                bool_to_int(cookies.secure),
                bool_to_int(cookies.http_only),
                cookies.name,
                cookies.creation_utc.parse::<u64>().unwrap(),
            ],
        )?;

        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// cookie表中添加数据
    /// WARN: (这里需要注意  插入数据库时  时间戳是同时生成的 也就是 creation_utc， last_access_utc， last_update_utc 字段是一样的值  不过不重要)
    pub fn add_cookie(path: &str, cookie: &str) -> Result<bool, ApplicationServerError> {
        let key = get_encrypt_key(path)?;
        let db = Db::new(cookie_file(path))?;
        let cookies: PluginsCookie = serde_json::from_str(cookie)?;

        let en_cookie_val: Vec<u8>;

        if cookies.value.is_empty() {
            en_cookie_val = Vec::new(); // 如果 cookies.value 为空，插入一个空的 Vec<u8>
        } else {
            en_cookie_val = enc_cookie(&cookies.value, &key)?; // 如果 cookies.value 不为空，插入加密后的值
        }

        let expiration: u64;

        if cookies.expiration_date == "0.0" {
            expiration = generate_nanosecond_timestamp();
        } else {
            expiration = seconds_to_timestamp(cookies.expiration_date.parse::<f64>().unwrap());
        }

        let sql = "
        INSERT INTO cookies (creation_utc, host_key, top_frame_site_key, name, value, encrypted_value, path, expires_utc, is_secure, is_httponly, last_access_utc, has_expires, is_persistent, priority, samesite, source_scheme, source_port, last_update_utc, source_type, has_cross_site_ancestor)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)
    ";

        let status = db.query_table(
            &sql,
            params![
                generate_nanosecond_timestamp(),
                cookies.domain,
                "".to_string(),
                cookies.name,
                "".to_string(),
                en_cookie_val,
                cookies.path,
                expiration,
                bool_to_int(cookies.secure),
                bool_to_int(cookies.http_only),
                generate_nanosecond_timestamp(),
                1,
                1,
                1,
                0, //这个要注意  samesite  有两个值 0 和 -1
                2,
                443,
                generate_nanosecond_timestamp(),
                1,
                1
            ],
        )?;

        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// cookie表删除数据
    pub fn delete_cookie(path: &str, creation: &str) -> Result<bool, ApplicationServerError> {
        let db = Db::new(cookie_file(path))?;

        let sql = "
        DELETE FROM cookies
        WHERE creation_utc = ?1
    ";
        let status = db.query_table(&sql, params![creation.parse::<u64>().unwrap()])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
