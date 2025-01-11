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
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<bool, Error> {
        let row = sqlx::query("delete from user_infos where id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
