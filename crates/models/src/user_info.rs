use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct UserInfo {
    pub id: u32,
    pub nickname: String,
    pub password: String,
    pub description: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl UserInfo {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, user: &UserInfo) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let row = sqlx::query("insert into user_infos (nickname, password, email) values(?, ?, ?)")
            .bind(&user.nickname)
            .bind(&user.password)
            .bind(&user.email)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_id(pool: &Pool<Sqlite>, id: i32) -> Result<UserInfo, Error> {
        let user: UserInfo = sqlx::query_as("select * from user_infos where id = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn query_email(pool: &Pool<Sqlite>, email: &str) -> Result<UserInfo, Error> {
        let user: UserInfo = sqlx::query_as("select * from user_infos where email = ?")
            .bind(email)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn query_nickname(pool: &Pool<Sqlite>, nickname: &str) -> Result<UserInfo, Error> {
        let user: UserInfo = sqlx::query_as("select * from user_infos where nickname = ?")
            .bind(nickname)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn query_by_col(
        pool: &Pool<Sqlite>,
        col_name: &str,
        col_value: &str,
        page_num: i32,
        page_size: i32,
    ) -> Result<(i64, Vec<UserInfo>), Error> {
        let (total,): (i64,) = sqlx::query_as("select count(1) from user_infos")
            .fetch_one(pool)
            .await?;
        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let users: Vec<UserInfo> = if col_name.is_empty() {
            sqlx::query_as("select * from user_infos limit ? offset ?")
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else {
            sqlx::query_as(&format!(
                "select * from user_infos where {} = ? limit ? offset ?",
                col_name
            ))
            .bind(col_value)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok((total, users))
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

        let row = sqlx::query(&format!("UPDATE user_infos SET {} = ? WHERE id = ?", col_name))
            .bind(col_value)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<bool, Error> {
        let row = sqlx::query("delete from user_infos where id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
