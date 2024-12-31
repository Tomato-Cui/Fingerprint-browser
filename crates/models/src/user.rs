use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

use crate::user_info::UserInfo;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct User {
    pub id: u32,
    pub uuid: String,
    pub user_info_id: u32,
    pub user_avatar_id: Option<u32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl User {
    #[allow(dead_code)]
    pub async fn insert(
        pool: &Pool<Sqlite>,
        uuid: &str,
        user_info: &UserInfo,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let user_info_id: u32 = sqlx::query_scalar(
            "insert into user_infos (nickname, password, email) values(?, ?, ?) returning id;",
        )
        .bind(&user_info.nickname)
        .bind(&user_info.password)
        .bind(&user_info.email)
        .bind(&user_info.description)
        .bind(&user_info.phone)
        .fetch_one(&mut *tx)
        .await?;

        let row = sqlx::query("insert into users (uuid, user_info_id) values(?, ?)")
            .bind(uuid)
            .bind(user_info_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn search_by_email(pool: &Pool<Sqlite>, email: &str) -> Result<Vec<String>, Error> {
        let select_sql = format!(
            "SELECT ui.email
            FROM users u
            JOIN user_infos ui ON u.user_info_id = ui.id
            WHERE ui.email like '%{}%' AND u.deleted_at IS NULL",
            email
        );

        let user_emails: Vec<String> = sqlx::query_scalar(&select_sql).fetch_all(pool).await?;

        Ok(user_emails)
    }

    #[allow(dead_code)]
    pub async fn query_uuid_by_email(pool: &Pool<Sqlite>, email: &str) -> Result<String, Error> {
        let user_uuid: String = sqlx::query_scalar(
            r#"
            SELECT u.uuid
            FROM users u
            JOIN user_infos ui ON u.user_info_id = ui.id
            WHERE ui.email = ?;"#,
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(user_uuid)
    }

    #[allow(dead_code)]
    pub async fn query_info_by_uuid(pool: &Pool<Sqlite>, uuid: i32) -> Result<UserInfo, Error> {
        let user: UserInfo = sqlx::query_as(
            r#"
            SELECT ui.*
            FROM users u
            JOIN user_infos ui ON u.user_info_id = ui.id
            WHERE u.uuid = ?;"#,
        )
        .bind(uuid)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn query_user_by_user_info_id(
        pool: &Pool<Sqlite>,
        user_info_id: u32,
    ) -> Result<User, Error> {
        let user: User = sqlx::query_as(
            r#"
        SELECT u.*
        FROM users u
        WHERE u.user_info_id = ? AND u.deleted_at IS NULL;"#,
        )
        .bind(user_info_id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn query_email(pool: &Pool<Sqlite>, email: &str) -> Result<UserInfo, Error> {
        let user: UserInfo = sqlx::query_as(
            r#"
            SELECT ui.*
            FROM user_infos ui
            WHERE ui.email = ?;"#,
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn query_nickname(pool: &Pool<Sqlite>, nickname: &str) -> Result<UserInfo, Error> {
        let user: UserInfo = sqlx::query_as(
            r#"
            SELECT ui.*
            FROM user_infos ui
            WHERE ui.nickname = ?;"#,
        )
        .bind(nickname)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn update_user_info(pool: &Pool<Sqlite>, user_info: UserInfo) -> Result<bool, Error> {
        let row = sqlx::query(
            r#"
            UPDATE user_infos
            SET
                nickname = ?,
                password = ?,
                description = ?,
                email = ?,
                phone = ?
            WHERE id = (
                SELECT user_info_id
                FROM users
                WHERE uuid = ?
            );"#,
        )
        .bind(user_info.nickname)
        .bind(user_info.password)
        .bind(user_info.description)
        .bind(user_info.email)
        .bind(user_info.phone)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn update_password(
        pool: &Pool<Sqlite>,
        email: &str,
        new_password: &str,
    ) -> Result<bool, Error> {
        let row = sqlx::query(
            r#"
        UPDATE user_infos
        SET
            password = ?
        WHERE email = ?;"#,
        )
        .bind(new_password)
        .bind(email)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, uuid: &str) -> Result<bool, Error> {
        let row = sqlx::query("update users set deleted_at = DATETIME('now') where uuid = ?")
            .bind(uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
