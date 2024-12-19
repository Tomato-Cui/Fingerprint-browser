use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl Group {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, group: &Group) -> Result<bool, Error> {
        let row = sqlx::query("insert into groups (name, description,owner_id) values(?, ?, ?)")
            .bind(group.name.to_string())
            .bind(group.description.clone().unwrap_or_default())
            .bind(group.owner_id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_id(pool: &Pool<Sqlite>, id: i32) -> Result<Group, Error> {
        let group: Group =
            sqlx::query_as("select * from groups where id = ? and deleted_at is null")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(group)
    }

    #[allow(dead_code)]
    pub async fn query_by_col(
        pool: &Pool<Sqlite>,
        col_name: &str,
        col_value: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Group>), Error> {
        let (total,): (i64,) = sqlx::query_as("select count(1) from groups")
            .fetch_one(pool)
            .await?;
        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let groups: Vec<Group> = if col_name.is_empty() {
            sqlx::query_as("select * from groups limit ? offset ?")
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else {
            sqlx::query_as(&format!(
                "select * from groups where {} = ? limit ? offset ?",
                col_name
            ))
            .bind(col_value)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok((total, groups))
    }

    #[allow(dead_code)]
    pub async fn update(pool: &Pool<Sqlite>, group: &Group) -> Result<bool, Error> {
        let row = sqlx::query(
            "update groups set name = ?, description = ?, updated_at = DATETIME('now') where id = ?",
        )
        .bind(&group.name)
        .bind(&group.description)
        .bind(group.id)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn update_by_col(
        pool: &Pool<Sqlite>,
        id: i32,
        col_name: &str,
        col_value: &str,
    ) -> Result<bool, Error> {
        if col_name.is_empty() {
            return Err(sqlx::error::Error::ColumnNotFound(format!(
                "{} column not found",
                col_name
            )));
        }
        let row = sqlx::query(&format!("UPDATE groups SET {} = ? WHERE id = ?", col_name))
            .bind(col_value)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<bool, Error> {
        let row = sqlx::query("delete from groups where id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
