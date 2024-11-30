use crate::{apis::PageParam, config::get_config, db::Db, errors::ApplicationServerError};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Group数据结构
#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(skip_deserializing)] // 忽略反序列化
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Group {
    /// 插入数据到group表
    pub fn insert_group(
        name: &str,
        description: Option<String>,
    ) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;
        let sql = "insert into groups (name, description, created_at) values(?1, ?2, ?3)";

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ApplicationServerError::SystemTimeError(e))?
            .as_secs();

        let status = db.query_table(sql, params![name, description, now])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 删除group表数据
    pub fn delete_group(id: i8) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;
        let sql = "delete from groups where id = (?1)";
        let status = db.query_table(sql, params![id])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 查询所有group表数据
    pub fn query_group(payload: PageParam) -> Result<Vec<Group>, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let page_num = payload.page_num.unwrap_or_else(|| 0);
        let page_size = payload.page_size.unwrap_or_else(|| 10);
        let offset = page_num * page_size;

        let sql = &format!(
            "select * from groups limit ({}) offset ({})",
            page_size, offset
        );

        let mut stmt = db.query_map_table(sql)?;
        let group_iter = stmt.query_map(params![], |row| {
            Ok(Group {
                id: row.get(0)?,          // 索引 0
                name: row.get(1)?,        // 索引 1
                description: row.get(2)?, // 索引 2
                created_at: row.get(3)?,  // 索引 3
                updated_at: row.get(3)?,  // 索引 4
            })
        })?;

        let group: Vec<Group> = group_iter.filter_map(Result::ok).collect();
        Ok(group)
    }

    /// 更新group表数据
    pub fn update_group(
        name: &str,
        description: Option<String>,
        id: i8,
    ) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ApplicationServerError::SystemTimeError(e))?
            .as_secs();

        let sql = "
        update groups 
            set name = ?1,
                description = ?2,
                updated_at = ?3
            where id = ?4";
        let status =
            db.query_table(sql, params![name, description.unwrap_or_default(), now, id])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
