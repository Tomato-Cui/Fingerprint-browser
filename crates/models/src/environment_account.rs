use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentAccount {
    pub id: Option<i32>,                      // 自增ID
    pub platform: String,                     // 平台名称
    pub platform_url: String,                 // 平台URL
    pub platform_description: Option<String>, // 平台描述
    pub environment_uuid: Option<i32>,        // 环境UUID
    pub user_uuid: Option<i32>,               // 用户UUID
    pub created_at: Option<String>,           // 创建时间
    pub updated_at: Option<String>,           // 更新时间
    pub deleted_at: Option<String>,           // 删除时间
}

impl EnvironmentAccount {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, account: &EnvironmentAccount) -> Result<bool, Error> {
        let sql = "
            INSERT INTO environment_accounts (
                platform, platform_url, platform_description, environment_uuid, user_uuid
            ) VALUES (
                ?, ?, ?, ?, ?
            )";

        let row = sqlx::query(sql)
            .bind(&account.platform)
            .bind(&account.platform_url)
            .bind(&account.platform_description)
            .bind(&account.environment_uuid)
            .bind(&account.user_uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_by_id(pool: &Pool<Sqlite>, id: u32) -> Result<EnvironmentAccount, Error> {
        let account: EnvironmentAccount = sqlx::query_as(
            "SELECT * FROM environment_accounts WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(account)
    }

    #[allow(dead_code)]
    pub async fn query_by_environment_uuid(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<EnvironmentAccount>), Error> {
        let (total,): (i64,) = sqlx::query_as("SELECT count(1) FROM environment_accounts WHERE environment_uuid = ? AND deleted_at IS NULL")
            .bind(environment_uuid)
            .fetch_one(pool)
            .await?;

        let offset = page_num * page_size;

        let accounts: Vec<EnvironmentAccount> = sqlx::query_as("SELECT * FROM environment_accounts WHERE environment_uuid = ? AND deleted_at IS NULL LIMIT ? OFFSET ?")
            .bind(environment_uuid)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok((total, accounts))
    }

    #[allow(dead_code)]
    pub async fn update(pool: &Pool<Sqlite>, account: &EnvironmentAccount) -> Result<bool, Error> {
        let sql = "
        UPDATE environment_accounts
            SET platform = ?, platform_url = ?, platform_description = ?, environment_uuid = ?, user_uuid = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND deleted_at IS NULL;
        ";

        let row = sqlx::query(sql)
            .bind(&account.platform)
            .bind(&account.platform_url)
            .bind(&account.platform_description)
            .bind(&account.environment_uuid)
            .bind(&account.user_uuid)
            .bind(account.id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let sql = "UPDATE environment_accounts SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";

        let row = sqlx::query(sql).bind(id).execute(pool).await?;

        Ok(row.rows_affected() == 1)
    }
}
