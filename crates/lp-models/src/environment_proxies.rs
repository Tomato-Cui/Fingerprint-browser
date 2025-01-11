use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

use crate::environment_proxy_group::ProxyGroup;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Proxy {
    pub id: Option<i32>,                   // 自增ID
    pub kind: String,                      // 类型
    pub host: String,                      // 主机
    pub port: String,                      // 端口
    pub username: Option<String>,          // 用户名
    pub password: Option<String>,          // 密码
    pub user_uuid: Option<String>,         // 用户UUID
    pub environment_group_id: Option<i32>, // 环境组ID
    pub created_at: Option<String>,        // 创建时间
    pub updated_at: Option<String>,        // 更新时间
    pub deleted_at: Option<String>,        // 删除时间
}

impl Proxy {
    #[allow(dead_code)]
    pub async fn insert_proxy(pool: &Pool<Sqlite>, proxy: &Proxy) -> Result<u32, Error> {
        let sql = "
        INSERT INTO environment_proxies (
            kind, host, port, username, password, user_uuid, environment_group_id
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?
        ) returning id";

        let row: u32 = sqlx::query_scalar(sql)
            .bind(&proxy.kind)
            .bind(&proxy.host)
            .bind(&proxy.port)
            .bind(&proxy.username)
            .bind(&proxy.password)
            .bind(&proxy.user_uuid)
            .bind(&proxy.environment_group_id)
            .fetch_one(pool)
            .await?;

        Ok(row)
    }

    #[allow(dead_code)]
    pub async fn query_proxy_by_id(
        pool: &Pool<Sqlite>,
        id: u32,
        user_uuid: &str,
    ) -> Result<Proxy, Error> {
        let proxy: Proxy = sqlx::query_as(
        "SELECT * FROM environment_proxies WHERE id = ? AND user_uuid = ? AND deleted_at IS NULL"
        )
        .bind(id)
        .bind(user_uuid)
        .fetch_one(pool)
        .await?;

        Ok(proxy)
    }

    #[allow(dead_code)]
    pub async fn query_proxies_by_group_id(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        proxy_group_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Proxy>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environment_proxies WHERE user_uuid = ? AND environment_group_id = ? and deleted_at IS NULL",
        )
        .bind(user_uuid)
        .bind(proxy_group_id)
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let proxies: Vec<Proxy> = sqlx::query_as(
            r#"
            SELECT * FROM environment_proxies 
                WHERE user_uuid = ? 
                    AND environment_group_id = ? 
                    and deleted_at IS NULL
            LIMIT ? OFFSET ?
        "#,
        )
        .bind(user_uuid)
        .bind(proxy_group_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, proxies))
    }

    #[allow(dead_code)]
    pub async fn query_proxies_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Proxy>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environment_proxies WHERE user_uuid = ? AND deleted_at IS NULL",
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

        let proxies: Vec<Proxy> = sqlx::query_as(
        "SELECT * FROM environment_proxies WHERE user_uuid = ? AND deleted_at IS NULL LIMIT ? OFFSET ?"
            )
        .bind(user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, proxies))
    }

    #[allow(dead_code)]
    pub async fn update_proxy(
        pool: &Pool<Sqlite>,
        id: u32,
        user_uuid: &str,
        proxy: &Proxy,
    ) -> Result<bool, Error> {
        let sql = "
        UPDATE environment_proxies
        SET kind = ?, host = ?, port = ?, username = ?, password = ?, environment_group_id = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ? and user_uuid = ? AND deleted_at IS NULL
    ";

        let row = sqlx::query(sql)
            .bind(&proxy.kind)
            .bind(&proxy.host)
            .bind(&proxy.port)
            .bind(&proxy.username)
            .bind(&proxy.password)
            .bind(&proxy.environment_group_id)
            .bind(id)
            .bind(user_uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn update_proxy_group_for_proxy(
        pool: &Pool<Sqlite>,
        proxy_id: u32,
        new_group_id: u32,
    ) -> Result<bool, Error> {
        let sql = "
        UPDATE environment_proxies
        SET environment_group_id = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND deleted_at IS NULL
    ";

        let row = sqlx::query(sql)
            .bind(new_group_id)
            .bind(proxy_id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_proxy_groups_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<ProxyGroup>), Error> {
        let (total,): (i64,) = sqlx::query_as(
        "SELECT count(DISTINCT environment_group_id) FROM environment_proxies WHERE user_uuid = ? AND deleted_at IS NULL"
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

        let group_ids: Vec<i32> = sqlx::query_as(
        "SELECT DISTINCT environment_group_id FROM environment_proxies WHERE user_uuid = ? AND deleted_at IS NULL LIMIT ? OFFSET ?"
        )
        .bind(user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|(group_id,)| group_id)
        .collect();

        let proxy_groups: Vec<ProxyGroup> = sqlx::query_as(
            "SELECT * FROM environment_proxy_groups WHERE id IN (?) AND deleted_at IS NULL",
        )
        .bind(
            group_ids
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(","),
        )
        .fetch_all(pool)
        .await?;

        Ok((total, proxy_groups))
    }

    #[allow(dead_code)]
    pub async fn delete_proxy(
        pool: &Pool<Sqlite>,
        id: u32,
        user_uuid: &str,
    ) -> Result<bool, Error> {
        let sql = "UPDATE environment_proxies SET deleted_at = CURRENT_TIMESTAMP WHERE id = ? and user_uuid = ?";

        let row = sqlx::query(sql)
            .bind(id)
            .bind(user_uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete_proxys(
        pool: &Pool<Sqlite>,
        id: Vec<u32>,
        user_uuid: &str,
    ) -> Result<bool, Error> {
        let ids = id
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let sql = format!(
            "UPDATE environment_proxies SET deleted_at = CURRENT_TIMESTAMP WHERE id IN ({}) AND user_uuid = ?",
            ids
        );

        let row = sqlx::query(&sql).bind(user_uuid).execute(pool).await?;

        Ok(row.rows_affected() >= 1)
    }
}
