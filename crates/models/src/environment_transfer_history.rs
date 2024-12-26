use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

use crate::environment::Environment;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentTransferHistory {
    pub id: Option<i32>,            // 自增ID
    pub environment_uuid: String,   // 环境UUID
    pub from_user_uuid: String,     // 转移发起者UUID
    pub to_user_uuid: String,       // 接收者UUID
    pub created_at: Option<String>, // 创建时间
    pub updated_at: Option<String>, // 更新时间
    pub deleted_at: Option<String>, // 删除时间
}

impl EnvironmentTransferHistory {
    #[allow(dead_code)]
    pub async fn insert_transfer_history(
        pool: &Pool<Sqlite>,
        transfer_history: &EnvironmentTransferHistory,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let sql = "
        INSERT INTO environment_transfer_history (
            environment_uuid, from_user_uuid, to_user_uuid
        ) VALUES (
            ?, ?, ?
        )";

        let row = sqlx::query(sql)
            .bind(&transfer_history.environment_uuid)
            .bind(&transfer_history.from_user_uuid)
            .bind(&transfer_history.to_user_uuid)
            .execute(&mut *tx)
            .await?;

        let sql = "
        UPDATE environments
        SET user_uuid = ?, updated_at = CURRENT_TIMESTAMP
        WHERE uuid = ?";

        sqlx::query(sql)
            .bind(&transfer_history.to_user_uuid)
            .bind(&transfer_history.environment_uuid)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_environments_by_from_user_uuid(
        pool: &Pool<Sqlite>,
        from_user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Environment>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments e
         JOIN environment_transfer_history eth ON e.uuid = eth.environment_uuid
         WHERE eth.from_user_uuid = ? AND e.deleted_at IS NULL AND eth.deleted_at IS NULL",
        )
        .bind(from_user_uuid)
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let environments: Vec<Environment> = sqlx::query_as(
            "SELECT e.* FROM environments e
         JOIN environment_transfer_history eth ON e.uuid = eth.environment_uuid
         WHERE eth.from_user_uuid = ? AND e.deleted_at IS NULL AND eth.deleted_at IS NULL
         LIMIT ? OFFSET ?",
        )
        .bind(from_user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, environments))
    }

    #[allow(dead_code)]
    pub async fn query_transfer_history_by_environment_uuid(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
    ) -> Result<EnvironmentTransferHistory, Error> {
        let transfer_history: EnvironmentTransferHistory = sqlx::query_as(
            "SELECT * FROM environment_transfer_history WHERE environment_uuid = ? AND deleted_at IS NULL",
        )
        .bind(environment_uuid)
        .fetch_one(pool)
        .await?;

        Ok(transfer_history)
    }

    #[allow(dead_code)]
    pub async fn query_all_transfer_histories(
        pool: &Pool<Sqlite>,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<EnvironmentTransferHistory>), Error> {
        // 获取总数
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environment_transfer_history WHERE deleted_at IS NULL",
        )
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        // 获取分页的转移历史记录列表
        let transfer_histories: Vec<EnvironmentTransferHistory> = sqlx::query_as(
            "SELECT * FROM environment_transfer_history WHERE deleted_at IS NULL LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, transfer_histories))
    }

    #[allow(dead_code)]
    pub async fn update_transfer_history(
        pool: &Pool<Sqlite>,
        transfer_history: &EnvironmentTransferHistory,
    ) -> Result<bool, Error> {
        let sql = "
            UPDATE environment_transfer_history
            SET environment_uuid = ?, from_user_uuid = ?, to_user_uuid = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND deleted_at IS NULL
        ";

        let row = sqlx::query(sql)
            .bind(&transfer_history.environment_uuid)
            .bind(&transfer_history.from_user_uuid)
            .bind(&transfer_history.to_user_uuid)
            .bind(transfer_history.id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete_transfer_history(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        environment_uuid: &str,
    ) -> Result<bool, Error> {
        let sql =
            "UPDATE environment_transfer_history SET deleted_at = CURRENT_TIMESTAMP WHERE environment_uuid = ? and to_user_uuid = ?";

        let row = sqlx::query(sql)
            .bind(environment_uuid)
            .bind(user_uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
