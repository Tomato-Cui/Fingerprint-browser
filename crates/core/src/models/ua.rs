/// user agent data models
use crate::{config::AConfig, db::Db, errors::ApplicationServerError};
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// Ua
#[derive(Debug, Serialize, Deserialize)]
pub struct Ua {
    #[serde(skip_deserializing)] // ignore deserializing
    pub id: i32,
    pub os_name: String,
    pub os_ver: String,
    pub browser_ver: String,
}
impl Ua {
    /// ua表中插入数据
    pub fn insert_ua(ua: &str) -> Result<bool, ApplicationServerError> {
        let ua_info: Ua = serde_json::from_str(ua)?;

        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "insert into ua_table (os_name, os_ver, browser_ver) values(?1, ?2, ?3)";
        let status = db.query_table(
            sql,
            params![ua_info.os_name, ua_info.os_ver, ua_info.browser_ver],
        )?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// ua表中删除数据
    pub fn delete_ua(id: i32) -> Result<bool, ApplicationServerError> {
        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "delete from ua_table where id = (?1)";
        let status = db.query_table(sql, params![id])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// uak查询所有数据
    pub fn query_ua() -> Result<String, ApplicationServerError> {
        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "select * from ua_table";
        let mut stmt = db.query_map_table(sql)?;
        let ua_iter = stmt.query_map(params![], |row| {
            Ok(Ua {
                id: row.get(0)?,          // 索引 0
                os_name: row.get(1)?,     // 索引 1
                os_ver: row.get(2)?,      // 索引 2
                browser_ver: row.get(3)?, // 索引 3
            })
        })?;

        let ua: Vec<Ua> = ua_iter.filter_map(Result::ok).collect();
        Ok(serde_json::to_string(&ua)?)
    }
}
