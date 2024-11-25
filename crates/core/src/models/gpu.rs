/// gpu data models
use crate::{config::AConfig, db::Db, errors::ApplicationServerError};
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// GPU数据结构
#[derive(Debug, Deserialize, Serialize)]
pub struct Gpu {
    #[serde(skip_deserializing)] // ignore deserializing
    pub id: i32,
    pub gpu_ven: String,
    pub gpu_rend: String,
    pub os_name: String,
}
impl Gpu {
    /// gpu表插入数据
    pub fn insert_gpu(gpu: &str) -> Result<bool, ApplicationServerError> {
        let gpu_info: Gpu = serde_json::from_str(gpu)?;
        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "insert into gpu_table (gpu_ven, gpu_rend, os_name) values(?1, ?2, ?3)";
        let status = db.query_table(
            sql,
            params![gpu_info.gpu_ven, gpu_info.gpu_rend, gpu_info.os_name],
        )?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    /// gpu表删除数据
    pub fn delete_gpu(id: i32) -> Result<bool, ApplicationServerError> {
        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "delete from gpu_table where id = (?1)";
        let status = db.query_table(sql, params![id])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// gpu表查询所有数据
    pub fn query_gpu() -> Result<String, ApplicationServerError> {
        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "select * from gpu_table";
        let mut stmt = db.query_map_table(sql)?;
        let gpu_iter = stmt.query_map(params![], |row| {
            Ok(Gpu {
                id: row.get(0)?,       // 索引 0
                gpu_ven: row.get(1)?,  // 索引 1
                gpu_rend: row.get(2)?, // 索引 2
                os_name: row.get(3)?,  // 索引 3
            })
        })?;

        let gpu: Vec<Gpu> = gpu_iter.filter_map(Result::ok).collect();
        Ok(serde_json::to_string(&gpu)?)
    }
}
