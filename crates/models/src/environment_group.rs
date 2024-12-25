use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentGroup {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl EnvironmentGroup {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, group: &EnvironmentGroup) -> Result<bool, Error> {
        let row = sqlx::query("INSERT INTO environment_groups (name, description) VALUES (?, ?)")
            .bind(&group.name)
            .bind(&group.description)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_by_id(pool: &Pool<Sqlite>, id: u32) -> Result<EnvironmentGroup, Error> {
        let group: EnvironmentGroup =
            sqlx::query_as("SELECT * FROM environment_groups WHERE id = ? AND deleted_at IS NULL")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(group)
    }

    #[allow(dead_code)]
    pub async fn query_groups_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<EnvironmentGroup>), Error> {
        let (total,): (i64,) = sqlx::query_as(
        "SELECT count(DISTINCT group_id) FROM environments WHERE user_uuid = ? AND deleted_at IS NULL"
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
            "SELECT DISTINCT group_id FROM environments WHERE user_uuid = ? AND deleted_at IS NULL LIMIT ? OFFSET ?",
        )
        .bind(user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|(group_id,)| group_id)
        .collect();

        let groups: Vec<EnvironmentGroup> = sqlx::query_as(
            "SELECT * FROM environment_groups WHERE id IN (?) AND deleted_at IS NULL",
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

        Ok((total, groups))
    }

    #[allow(dead_code)]
    pub async fn update(pool: &Pool<Sqlite>, group: &EnvironmentGroup) -> Result<bool, Error> {
        let row = sqlx::query(
            "UPDATE environment_groups SET name = ?, description = ?, updated_at = DATETIME('now') WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(&group.name)
        .bind(&group.description)
        .bind(group.id)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let row =
            sqlx::query("UPDATE environment_groups SET deleted_at = DATETIME('now') WHERE id = ?")
                .bind(id)
                .execute(pool)
                .await?;

        Ok(row.rows_affected() == 1)
    }
}
