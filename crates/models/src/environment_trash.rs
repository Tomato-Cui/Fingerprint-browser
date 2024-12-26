use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

use crate::environment::Environment;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentTrash {
    pub id: i32, // 自增ID
    pub environment_uuid: String,
    pub from_user_uuid: String,
    pub created_at: Option<String>, // 创建时间
    pub updated_at: Option<String>, // 更新时间
    pub deleted_at: Option<String>, // 删除时间
}

impl EnvironmentTrash {
    #[allow(dead_code)]
    pub async fn query_deleted_environments_by_environment_uuid(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
    ) -> Result<Environment, Error> {
        let environment: Environment = sqlx::query_as(
            "SELECT * FROM environments WHERE environment_uuid  = ? AND deleted_at IS NOT NULL",
        )
        .bind(environment_uuid)
        .fetch_one(pool)
        .await?;

        Ok(environment)
    }

    #[allow(dead_code)]
    pub async fn query_deleted_environments_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Environment>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments WHERE user_uuid = ? AND deleted_at IS NOT NULL",
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

        let environments: Vec<Environment> = sqlx::query_as(
        "SELECT * FROM environments WHERE user_uuid = ? AND deleted_at IS NOT NULL LIMIT ? OFFSET ?",
        )
        .bind(user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, environments))
    }

    #[allow(dead_code)]
    pub async fn restore_deleted_environment(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        environment_uuid: &str,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let update_row = sqlx::query(
            "UPDATE environments SET deleted_at = NULL WHERE uuid = ? and from_user_uuid = ?",
        )
        .bind(environment_uuid)
        .bind(user_uuid)
        .execute(&mut *tx)
        .await?;

        let delete_row = sqlx::query(
            "DELETE FROM environment_trash WHERE environment_uuid = ? and from_user_uuid = ?",
        )
        .bind(environment_uuid)
        .bind(user_uuid)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(update_row.rows_affected() > 0 && delete_row.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn restore_deleted_environments(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        environment_uuids: &[&str],
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let update_query = format!(
            "UPDATE environments SET deleted_at = NULL WHERE uuid IN ({}) where from_user_uuid = ?",
            environment_uuids
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut update_stmt = sqlx::query(&update_query);
        for uuid in environment_uuids {
            update_stmt = update_stmt.bind(uuid);
        }
        update_stmt = update_stmt.bind(user_uuid);

        let update_row = update_stmt.execute(&mut *tx).await?;

        let delete_query = format!(
            "DELETE FROM environment_trash WHERE environment_uuid IN ({}) and from_user_uuid = ?",
            environment_uuids
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut delete_stmt = sqlx::query(&delete_query);
        for uuid in environment_uuids {
            delete_stmt = delete_stmt.bind(uuid);
        }

        delete_stmt = delete_stmt.bind(user_uuid);
        let delete_row = delete_stmt.execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(update_row.rows_affected() > 0 && delete_row.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn restore_all_deleted_environments_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let update_row = sqlx::query(
                "UPDATE environments SET deleted_at = NULL WHERE user_uuid = ? AND deleted_at IS NOT NULL"
            )
            .bind(user_uuid)
            .execute(&mut *tx)
            .await?;

        let delete_row = sqlx::query("DELETE FROM environment_trash WHERE from_user_uuid = ?")
            .bind(user_uuid)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(update_row.rows_affected() > 0 && delete_row.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn permanently_delete_all_deleted_environments_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let delete_row = sqlx::query("DELETE FROM environment_trash WHERE from_user_uuid = ?")
            .bind(user_uuid)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(delete_row.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn permanently_delete_environments(
        pool: &Pool<Sqlite>,
        environment_uuids: &[&str],
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let delete_query = format!(
            "DELETE FROM environment_trash WHERE environment_uuid IN ({})",
            environment_uuids
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut delete_stmt = sqlx::query(&delete_query);
        for uuid in environment_uuids {
            delete_stmt = delete_stmt.bind(uuid);
        }

        let delete_row = delete_stmt.execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(delete_row.rows_affected() > 0)
    }
}
