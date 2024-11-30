/// user agent data models
use crate::{config::get_config, db::Db, errors::ApplicationServerError, utils::common::to_string};
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
    pub fn insert_ua(ua: Ua) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;
        let sql = "insert into ua_table (os_name, os_ver, browser_ver) values(?1, ?2, ?3)";
        let status = db.query_table(sql, params![ua.os_name, ua.os_ver, ua.browser_ver])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// ua表中删除数据
    pub fn delete_ua(id: i8) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;
        let sql = "delete from ua_table where id = (?1)";
        let status = db.query_table(sql, params![id])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// ua查询所有数据
    pub fn query_ua() -> Result<Vec<Ua>, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;
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
        Ok(ua)
    }

    /// ua更新数据
    pub fn update_ua(ua: Ua, id: i8) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let sql = "
            UPDATE ua_table
            SET os_name = ?1,
                os_ver = ?2,
                browser_ver = ?3,
            WHERE id = ?4
        ";
        let status = db.query_table(sql, params![ua.os_name, ua.os_ver, ua.browser_ver, id])?;
        if status == 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn to_string(&self) -> Result<String, ApplicationServerError> {
        Ok(format!(
            "{} {} {}",
            self.os_name, self.browser_ver, self.os_ver
        ))
    }

    pub fn string_to(&self) -> Result<String, ApplicationServerError> {
        to_string(self)
    }
}

impl From<String> for Ua {
    fn from(value: String) -> Self {
        Self {
            id: 0,
            os_name: "".to_string(),
            os_ver: "".to_string(),
            browser_ver: value,
        }
    }
}
