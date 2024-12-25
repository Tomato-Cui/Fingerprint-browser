use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct UserAvatars {
    pub id: u32,
    pub href: String,
    pub path: String,
    pub hash: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl UserAvatars {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, payload: &UserAvatars) -> Result<bool, Error> {
        let row = sqlx::query("insert into user_avatars (href, path, hash) values(?, ?, ?)")
            .bind(&payload.href)
            .bind(&payload.path)
            .bind(&payload.path)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_id(pool: &Pool<Sqlite>, id: i32) -> Result<UserAvatars, Error> {
        let user: UserAvatars = sqlx::query_as("select * from user_avatars where id = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn query_hash(pool: &Pool<Sqlite>, id: i32) -> Result<UserAvatars, Error> {
        let user: UserAvatars = sqlx::query_as("select * from user_avatars where hash = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<bool, Error> {
        let row = sqlx::query("delete from user_avatars where id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
