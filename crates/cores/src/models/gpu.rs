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
    pub fn insert_gpu(gpu: Gpu) -> Result<bool, ApplicationServerError> {
        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "insert into gpu_table (gpu_ven, gpu_rend, os_name) values(?1, ?2, ?3)";
        let status = db.query_table(sql, params![gpu.gpu_ven, gpu.gpu_rend, gpu.os_name])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    /// gpu表删除数据
    pub fn delete_gpu(id: i8) -> Result<bool, ApplicationServerError> {
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
    pub fn query_gpu() -> Result<Vec<Gpu>, ApplicationServerError> {
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
        Ok(gpu)
    }

    /// gpu表更新数据
    pub fn update_gpu(gpu: Gpu, id: i8) -> Result<bool, ApplicationServerError> {
        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "
        update gpu_table 
            set gpu_ven = ?1,
                gpu_rend = ?2,
                os_name = ?3, 
            where id = ?4";
        let status = db.query_table(sql, params![gpu.gpu_ven, gpu.gpu_rend, gpu.os_name, id])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
