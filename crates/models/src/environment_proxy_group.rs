use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct ProxyGroup {
    pub id: Option<i32>,             // 自增ID
    pub name: String,                // 分组名称
    pub user_uuid: String,           // 用户uuid
    pub description: Option<String>, // 分组描述
    pub created_at: Option<String>,  // 创建时间
    pub updated_at: Option<String>,  // 更新时间
    pub deleted_at: Option<String>,  // 删除时间
}

impl ProxyGroup {
    #[allow(dead_code)]
    pub async fn insert_proxy_group(
        pool: &Pool<Sqlite>,
        proxy_group: &ProxyGroup,
    ) -> Result<bool, Error> {
        let sql = "
            INSERT INTO environment_proxy_groups (
                name, description
            ) VALUES (
                ?, ?
            )";

        let row = sqlx::query(sql)
            .bind(&proxy_group.name)
            .bind(&proxy_group.description)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_proxy_group_by_id(
        pool: &Pool<Sqlite>,
        id: u32,
    ) -> Result<ProxyGroup, Error> {
        let proxy_group: ProxyGroup = sqlx::query_as(
            "SELECT * FROM environment_proxy_groups WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(proxy_group)
    }

    #[allow(dead_code)]
    pub async fn query_all_proxy_groups_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<ProxyGroup>), Error> {
        // 获取总数
        let (total,): (i64,) = sqlx::query_as(
        "SELECT count(1) FROM environment_proxy_groups WHERE user_uuid = ? AND deleted_at IS NULL"
        )
        .bind(user_uuid)
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        // 获取分页的代理组列表
        let proxy_groups: Vec<ProxyGroup> = sqlx::query_as(
        "SELECT * FROM environment_proxy_groups WHERE user_uuid = ? AND deleted_at IS NULL LIMIT ? OFFSET ?"
        )
        .bind(user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, proxy_groups))
    }

    #[allow(dead_code)]
    pub async fn update_proxy_group(
        pool: &Pool<Sqlite>,
        id: u32,
        proxy_group: &ProxyGroup,
    ) -> Result<bool, Error> {
        let sql = "
            UPDATE environment_proxy_groups
            SET name = ?, description = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND deleted_at IS NULL
        ";

        let row = sqlx::query(sql)
            .bind(&proxy_group.name)
            .bind(&proxy_group.description)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete_proxy_group(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let sql = "UPDATE environment_proxy_groups SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";

        let row = sqlx::query(sql).bind(id).execute(pool).await?;

        Ok(row.rows_affected() == 1)
    }
}
