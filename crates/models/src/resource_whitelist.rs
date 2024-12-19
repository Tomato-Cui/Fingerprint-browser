use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct ResourceWhiteList {
    pub id: i32,
    pub href: String,
    pub method: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl ResourceWhiteList {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, resource: &ResourceWhiteList) -> Result<bool, Error> {
        let row = sqlx::query(
            "insert into resource_whitelist(href, method, description) values(?, ?, ?)",
        )
        .bind(resource.href.to_string())
        .bind(resource.method.to_string())
        .bind(resource.deleted_at.clone())
        .execute(pool)
        .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_id(pool: &Pool<Sqlite>, id: i32) -> Result<ResourceWhiteList, Error> {
        let resource: ResourceWhiteList =
            sqlx::query_as("select * from resource_whitelist where id = ?")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(resource)
    }

    #[allow(dead_code)]
    pub async fn query_by_col(
        pool: &Pool<Sqlite>,
        col_name: &str,
        col_value: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<ResourceWhiteList>), Error> {
        let (total,): (i64,) = sqlx::query_as("select count(1) from resource_whitelist")
            .fetch_one(pool)
            .await?;
        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let resources: Vec<ResourceWhiteList> = if col_name.is_empty() {
            sqlx::query_as("select * from resource_whitelist limit ? offset ?")
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else {
            sqlx::query_as(&format!(
                "select * from resource_whitelist where {} = ? limit ? offset ?",
                col_name
            ))
            .bind(col_value)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok((total, resources))
    }

    #[allow(dead_code)]
    pub async fn query_by_href_method(
        pool: &Pool<Sqlite>,
        href: &str,
        description: &str,
    ) -> Result<ResourceWhiteList, Error> {
        let resource: ResourceWhiteList =
            sqlx::query_as("select * from resource_whitelist where href = ? and method = ?")
                .bind(href)
                .bind(description)
                .fetch_one(pool)
                .await?;

        Ok(resource)
    }

    #[allow(dead_code)]
    pub async fn update(pool: &Pool<Sqlite>, resource: &ResourceWhiteList) -> Result<bool, Error> {
        let row = sqlx::query(
            "update resource_whitelist set href = ?, method = ?, description = ?, updated_at = DATETIME('now') where id = ?",
        )
        .bind(&resource.href)
        .bind(&resource.method)
        .bind(resource.description.clone())
        .bind(resource.id)
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
        let row = sqlx::query(&format!(
            "UPDATE resource_whitelist SET {} = ? WHERE id = ?",
            col_name
        ))
        .bind(col_value)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<bool, Error> {
        let row = sqlx::query("delete from resource_whitelist where id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
