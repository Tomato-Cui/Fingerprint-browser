use crate::{apis::PageParam, database::get_db, errors::ApplicationServerError};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Group数据结构
#[derive(Debug, Deserialize, Serialize, FromRow)]
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
    pub async fn insert_group(
        name: &str,
        description: Option<String>,
    ) -> Result<bool, ApplicationServerError> {
        let row =
            sqlx::query("insert into groups (name, description, created_at) values(?1, ?2, ?3)")
                .bind(name)
                .bind(description)
                .execute(get_db()?)
                .await?;

        Ok(row.rows_affected() == 1)
    }

    /// 删除group表数据
    pub async fn delete_group(id: i8) -> Result<bool, ApplicationServerError> {
        let row = sqlx::query("delete from groups where id = ?1")
            .bind(id)
            .execute(get_db()?)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    /// 查询所有group表数据
    pub async fn query_group(payload: PageParam) -> Result<Vec<Group>, ApplicationServerError> {
        let page_num = payload.page_num.unwrap_or_else(|| 0);
        let page_size = payload.page_size.unwrap_or_else(|| 10);
        let offset = page_num * page_size;

        let rows: Vec<Group> = sqlx::query_as("select * from groups limit ?1 offset ?2")
            .bind(page_size)
            .bind(offset)
            .fetch_all(get_db()?)
            .await?;

        Ok(rows)
    }

    /// 更新group表数据
    pub async fn update_group(
        name: &str,
        description: Option<String>,
        id: i8,
    ) -> Result<bool, ApplicationServerError> {
        let row = sqlx::query(
            "update groups set name = ?1, description = ?2, updated_at = now() where id = ?3",
        )
        .bind(name)
        .bind(description)
        .bind(id)
        .execute(get_db()?)
        .await?;

        Ok(row.rows_affected() == 1)
    }
}
