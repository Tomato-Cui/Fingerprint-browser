use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Proxy {
    pub id: i32,
    pub kind: String,
    pub value: String,
    pub environment_id: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl Proxy {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, proxy: &Proxy) -> Result<bool, Error> {
        let row = sqlx::query("insert into proxies (kind, value) values(?, ?)")
            .bind(&proxy.kind)
            .bind(&proxy.value)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_id(pool: &Pool<Sqlite>, id: i32) -> Result<Proxy, Error> {
        let proxy: Proxy = sqlx::query_as("select * from proxies where id = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(proxy)
    }

    #[allow(dead_code)]
    pub async fn query_by_col(
        pool: &Pool<Sqlite>,
        col_name: &str,
        col_value: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Proxy>), Error> {
        let (total,): (i64,) = sqlx::query_as("select count(1) from proxies")
            .fetch_one(pool)
            .await?;
        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let proxys: Vec<Proxy> = if col_name.is_empty() {
            sqlx::query_as("select * from proxies limit ? offset ?")
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else {
            sqlx::query_as(&format!(
                "select * from proxies where {} = ? limit ? offset ?",
                col_name
            ))
            .bind(col_value)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok((total, proxys))
    }

    #[allow(dead_code)]
    pub async fn update(pool: &Pool<Sqlite>, proxy: &Proxy) -> Result<bool, Error> {
        let row = sqlx::query(
            "update proxies set kind = ?1, value = ?2, updated_at = DATETIME('now') where id = ?3",
        )
        .bind(&proxy.kind)
        .bind(&proxy.value)
        .bind(proxy.id)
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
            "UPDATE proxies SET {} = ?2 WHERE id = ?3",
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
        let row = sqlx::query("delete from proxies where id = ?1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
