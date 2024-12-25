use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentCookie {
    pub id: Option<i32>,               // 自增ID
    pub key: String,                   // Cookie键
    pub value: String,                 // Cookie值
    pub environment_uuid: Option<i32>, // 环境UUID
    pub created_at: Option<String>,    // 创建时间
    pub updated_at: Option<String>,    // 更新时间
    pub deleted_at: Option<String>,    // 删除时间
}

impl EnvironmentCookie {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, cookie: &EnvironmentCookie) -> Result<bool, Error> {
        let sql = "
            INSERT INTO environment_cookies (
                key, value, environment_uuid
            ) VALUES (
                ?, ?, ?
            )";

        let row = sqlx::query(sql)
            .bind(&cookie.key)
            .bind(&cookie.value)
            .bind(&cookie.environment_uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_by_id(pool: &Pool<Sqlite>, id: u32) -> Result<EnvironmentCookie, Error> {
        let cookie: EnvironmentCookie =
            sqlx::query_as("SELECT * FROM environment_cookies WHERE id = ? AND deleted_at IS NULL")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(cookie)
    }

    #[allow(dead_code)]
    pub async fn query_by_environment_uuid(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<EnvironmentCookie>), Error> {
        let (total,): (i64,) = sqlx::query_as("SELECT count(1) FROM environment_cookies WHERE environment_uuid = ? AND deleted_at IS NULL")
            .bind(environment_uuid)
            .fetch_one(pool)
            .await?;

        let offset = page_num * page_size;

        let cookies: Vec<EnvironmentCookie> = sqlx::query_as("SELECT * FROM environment_cookies WHERE environment_uuid = ? AND deleted_at IS NULL LIMIT ? OFFSET ?")
            .bind(environment_uuid)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok((total, cookies))
    }

    #[allow(dead_code)]
    pub async fn update(pool: &Pool<Sqlite>, cookie: &EnvironmentCookie) -> Result<bool, Error> {
        let sql = "
        UPDATE environment_cookies
            SET key = ?, value = ?, environment_uuid = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND deleted_at IS NULL;
        ";

        let row = sqlx::query(sql)
            .bind(&cookie.key)
            .bind(&cookie.value)
            .bind(&cookie.environment_uuid)
            .bind(cookie.id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let sql = "UPDATE environment_cookies SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";

        let row = sqlx::query(sql).bind(id).execute(pool).await?;

        Ok(row.rows_affected() == 1) // 如果删除了 1 行则返回 true
    }
}
