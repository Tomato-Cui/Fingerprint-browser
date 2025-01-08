use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::FromRow;
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct OperationLog {
    pub id: Option<u32>,
    pub team_id: Option<u32>,
    pub user_uuid: Option<String>,
    pub allowed_operation_id: u32,
    pub operation_status: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl OperationLog {
    pub async fn insert(
        pool: &SqlitePool,
        team_id: u32,
        user_uuid: &str,
        operation_log: OperationLog,
    ) -> Result<bool, Error> {
        let log = sqlx::query(
            "INSERT INTO operation_logs 
             (team_id, user_uuid, allowed_operation_id, operation_status) 
             VALUES (?, ?, ?, ?, ?, ?) 
             RETURNING *",
        )
        .bind(team_id)
        .bind(user_uuid)
        .bind(&operation_log.allowed_operation_id)
        .bind(&operation_log.operation_status)
        .execute(pool)
        .await?;

        Ok(log.rows_affected() >= 1)
    }

    pub async fn query_by_id(pool: &SqlitePool, id: u32) -> Result<OperationLog, Error> {
        let log = sqlx::query_as::<_, OperationLog>("SELECT * FROM operation_logs WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(log)
    }

    pub async fn query_by_user_uuid(
        pool: &SqlitePool,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<OperationLog>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT * FROM operation_logs WHERE user_uuid = ? AND deleted_at IS NULL",
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

        let logs: Vec<OperationLog> = sqlx::query_as(
            "SELECT * FROM operation_logs WHERE user_uuid = ? AND deleted_at IS NULL LIMIT ? OFFSET ?",
        )
        .bind(user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, logs))
    }

    pub async fn query_by_team_id(
        pool: &SqlitePool,
        team_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<OperationLog>), Error> {
        let (total,): (i64,) =
            sqlx::query_as("SELECT * FROM operation_logs WHERE team_id = ? AND deleted_at IS NULL")
                .bind(team_id)
                .fetch_one(pool)
                .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let logs: Vec<OperationLog> = sqlx::query_as(
            "SELECT * FROM operation_logs WHERE team_id = ? AND deleted_at IS NULL LIMIT ? OFFSET ?",
        )
        .bind(team_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, logs))
    }

    pub async fn update_status(pool: &SqlitePool, id: u32, status: u8) -> Result<bool, Error> {
        let row = sqlx::query(
            "UPDATE operation_logs 
             SET operation_status = ?, updated_at = CURRENT_TIMESTAMP 
             WHERE id = ? 
             RETURNING *",
        )
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() >= 1)
    }

    pub async fn delete(pool: &SqlitePool, id: u32) -> Result<bool, Error> {
        let row = sqlx::query(
            "UPDATE operation_logs 
             SET deleted_at = CURRENT_TIMESTAMP 
             WHERE id = ?",
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() >= 1)
    }
}
