use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentAccount {
    pub id: Option<i32>,      // 自增ID
    pub platform: String,     // 平台名称
    pub platform_url: String, // 平台URL
    pub platform_account: String,
    pub platform_password: String,
    pub platform_description: Option<String>, // 平台描述
    pub environment_uuid: String,             // 环境UUID
    pub user_uuid: Option<String>,                    // 用户UUID
    pub created_at: Option<String>,           // 创建时间
    pub updated_at: Option<String>,           // 更新时间
    pub deleted_at: Option<String>,           // 删除时间
}

impl EnvironmentAccount {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, account: &EnvironmentAccount) -> Result<bool, Error> {
        let sql = "
            INSERT INTO environment_accounts (
                platform, platform_url, platform_account, platform_password, platform_description, environment_uuid, user_uuid
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?
            )";

        let row = sqlx::query(sql)
            .bind(&account.platform)
            .bind(&account.platform_url)
            .bind(&account.platform_account)
            .bind(&account.platform_password)
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
    pub async fn query_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<EnvironmentAccount>), Error> {
        let environment_uuids: Vec<String> =
            sqlx::query_scalar("SELECT uuid FROM environments WHERE user_uuid = ?")
                .bind(user_uuid)
                .fetch_all(pool)
                .await?;

        let environmnet_uuids_str = environment_uuids
            .iter()
            .map(|v| format!("'{}'", v))
            .collect::<Vec<String>>()
            .join(",");

        let (total,): (i64,) = 
            sqlx::query_as(
            &format!("SELECT count(1) FROM environment_accounts WHERE environment_uuid in ({}) AND deleted_at IS NULL",environmnet_uuids_str)
            )
            .fetch_one(pool)
            .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let accounts: Vec<EnvironmentAccount> = 
            sqlx::query_as(
            &format!("SELECT * FROM environment_accounts WHERE environment_uuid in ({}) AND deleted_at IS NULL LIMIT ? OFFSET ?", environmnet_uuids_str)
            )
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok((total, accounts))
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

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
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
            SET 
                platform = ?,
                platform_url = ?,
                platform_account = ?,
                platform_password = ?,
                platform_description = ?,
                environment_uuid = ?,
                user_uuid = ?
            WHERE id = ? AND deleted_at IS NULL;
        ";

        let row = sqlx::query(sql)
            .bind(&account.platform)
            .bind(&account.platform_url)
            .bind(&account.platform_account)
            .bind(&account.platform_password)
            .bind(&account.platform_description)
            .bind(&account.environment_uuid)
            .bind(&account.user_uuid)
            .bind(&account.id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() >= 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let sql = "UPDATE environment_accounts SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";

        let row = sqlx::query(sql).bind(id).execute(pool).await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn deletes(pool: &Pool<Sqlite>, id: Vec<u32>) -> Result<bool, Error> {
        let ids = id
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let sql = format!(
            "UPDATE environment_accounts SET deleted_at = CURRENT_TIMESTAMP WHERE id IN ({})",
            ids
        );

        let row = sqlx::query(&sql).execute(pool).await?;

        Ok(row.rows_affected() >= 1)
    }
}
