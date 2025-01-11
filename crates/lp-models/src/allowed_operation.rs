use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::FromRow;
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct AllowedOperation {
    pub id: Option<u32>,
    pub operation_name: String,
    pub operation_description: Option<String>,
    pub resource_name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl AllowedOperation {
    pub async fn insert(
        pool: &SqlitePool,
        allowed_operation: AllowedOperation,
    ) -> Result<bool, Error> {
        let row = sqlx::query(
            "INSERT INTO allowed_operations (operation_name, operation_description, resource_name) 
             VALUES (?, ?, ?)",
        )
        .bind(allowed_operation.operation_name)
        .bind(allowed_operation.operation_description)
        .bind(allowed_operation.resource_name)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() >= 1)
    }

    pub async fn query_by_id(pool: &SqlitePool, id: u32) -> Result<AllowedOperation, Error> {
        let operation: AllowedOperation =
            sqlx::query_as("SELECT * FROM allowed_operations WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(operation)
    }

    pub async fn query_by_resource_name(
        pool: &SqlitePool,
        resource_name: &str,
    ) -> Result<Option<AllowedOperation>, Error> {
        let operation: Option<AllowedOperation> =
            sqlx::query_as("SELECT * FROM allowed_operations WHERE resource_name = ?")
                .bind(resource_name)
                .fetch_optional(pool)
                .await?;

        Ok(operation)
    }

    pub async fn exist_by_resource_name(
        pool: &SqlitePool,
        resource_name: &str,
    ) -> Result<bool, Error> {
        let count: u32 =
            sqlx::query_scalar("SELECT count(*) FROM allowed_operations WHERE resource_name = ?")
                .bind(resource_name)
                .fetch_one(pool)
                .await?;

        Ok(count >= 1)
    }

    pub async fn update(
        pool: &SqlitePool,
        id: u32,
        allowed_operation: AllowedOperation,
    ) -> Result<bool, Error> {
        let row = sqlx::query(
            "UPDATE allowed_operations 
             SET operation_name = ?, operation_description = ?, resource_name = ?, updated_at = CURRENT_TIMESTAMP 
             WHERE id = ?",
        )
        .bind(allowed_operation.operation_name)
        .bind(allowed_operation.operation_description)
        .bind(allowed_operation.resource_name)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() >= 1)
    }

    pub async fn delete(pool: &SqlitePool, id: u32) -> Result<bool, Error> {
        let row = sqlx::query("DELETE FROM allowed_operations WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() >= 1)
    }
}
