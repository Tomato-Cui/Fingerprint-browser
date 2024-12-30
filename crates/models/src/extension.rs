use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Sqlite};

use crate::{dto::extension, environment::Environment};

#[derive(Debug, Deserialize, Serialize, FromRow, Default)]
pub struct Extension {
    pub id: Option<i32>,
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub release_url: Option<String>,
    pub size: Option<i32>,
    pub all_can_use: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl Extension {
    pub async fn insert(pool: &Pool<Sqlite>, extension: Extension) -> Result<bool, Error> {
        let _result = sqlx::query(
            r#"
            INSERT INTO extensions (uuid, name, description, avatar_url, release_url, size, all_can_use)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&extension.uuid)
        .bind(&extension.name)
        .bind(&extension.description)
        .bind(&extension.avatar_url)
        .bind(&extension.release_url)
        .bind(&extension.size)
        .bind(&extension.all_can_use)
        .execute(pool)
        .await?;

        Ok(_result.rows_affected() >= 1)
    }

    pub async fn team_insert(
        pool: &Pool<Sqlite>,
        team_id: &str,
        extension_uuid: &str,
    ) -> Result<bool, Error> {
        let row = sqlx::query(
            "INSERT INTO team_extension_relation (team_id, extension_uuid) VALUES (?, ?)",
        )
        .bind(team_id)
        .bind(extension_uuid)
        .execute(pool)
        .await?;
        Ok(row.rows_affected() >= 1)
    }

    pub async fn user_insert(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        extension_uuid: &str,
    ) -> Result<bool, Error> {
        let row = sqlx::query(
            "INSERT INTO user_extension_relation (user_uuid, extension_uuid, open) VALUES (?, ?, 0)",
        )
        .bind(user_uuid)
        .bind(extension_uuid)
        .execute(pool)
        .await?;
        Ok(row.rows_affected() >= 1)
    }

    pub async fn query_by_team_id(
        pool: &Pool<Sqlite>,
        team_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Extension>), Error> {
        let total: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM team_extension_relation ter WHERE ter.team_id = ? and deleted_at is null",
        )
        .bind(team_id)
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let result: Vec<Extension> = sqlx::query_as(
            r#"
            SELECT e.* FROM extensions e 
                join team_extension_relation ter 
                    on e.uuid = ter.extension_uuid  
                WHERE ter.team_id = ? and deleted_at is null
                LIMIT ? OFFSET ?
            "#,
        )
        .bind(team_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, result))
    }

    pub async fn query_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<extension::UserExtensionInfo>), Error> {
        let total: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM user_extension_relation WHERE user_uuid = ? and deleted_at is null",
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

        let result: Vec<extension::UserExtensionInfo> = sqlx::query_as(
            r#"
            SELECT e.*, uer.open as open FROM extensions e 
                join user_extension_relation uer 
                    on e.uuid = uer.extension_uuid  
                WHERE uer.user_uuid = ? and uer.deleted_at is null
                LIMIT ? OFFSET ?
            "#,
        )
        .bind(user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, result))
    }

    pub async fn query_environmnet_uuid_by_extension_uuid(
        pool: &Pool<Sqlite>,
        extension_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Environment>), Error> {
        let total: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM environment_extension_relation eer WHERE eer.extension_uuid = ? and deleted_at is null",
        )
        .bind(extension_uuid)
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let result: Vec<Environment> = sqlx::query_as(
            r#"
            SELECT e2.* FROM extensions e 
                join environment_extension_relation eer 
                    on e.uuid = eer.extension_uuid  
                join environments e2 on eer.environment_uuid = e2.uuid
                WHERE eer.extension_uuid = ? and eer.deleted_at is null
                LIMIT ? OFFSET ?
            "#,
        )
        .bind(extension_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, result))
    }

    pub async fn query_by_environmnet_uuid(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Extension>), Error> {
        let total: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM environment_extension_relation eer WHERE eer.environment_uuid = ? and deleted_at is null",
        )
        .bind(environment_uuid)
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let result: Vec<Extension> = sqlx::query_as(
            r#"
            SELECT e.* FROM extensions e
                    join environment_extension_relation eer
                        on e.uuid = eer.extension_uuid
            WHERE eer.environment_uuid = ? and e.deleted_at is null
            LIMIT ? OFFSET ?;
            "#,
        )
        .bind(environment_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, result))
    }

    pub async fn query(
        pool: &Pool<Sqlite>,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Extension>), Error> {
        let total: i64 =
            sqlx::query_scalar("SELECT count(1) FROM extensions WHERE deleted_at is null")
                .fetch_one(pool)
                .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let result: Vec<Extension> = sqlx::query_as(
            r#"
            SELECT * FROM extensions 
                WHERE deleted_at is null
                LIMIT ? OFFSET ?
            "#,
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, result))
    }

    pub async fn update(
        pool: &Pool<Sqlite>,
        extension_uuid: &str,
        extension: Extension,
    ) -> Result<bool, Error> {
        let result = sqlx::query(
            r#"
            UPDATE extensions
            SET name = ?, description = ?, avatar_url = ?, release_url = ?, size = ?, updated_at = CURRENT_TIMESTAMP
            WHERE uuid = ?
            "#
        )
        .bind(&extension.name)
        .bind(&extension.description)
        .bind(&extension.avatar_url )
        .bind(&extension.release_url)
        .bind(&extension.size)
        .bind(extension_uuid)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() >= 1)
    }

    pub async fn user_toggle_extension(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        extension_uuid: &str,
        open: bool,
    ) -> Result<bool, Error> {
        let open = if open { 1 } else { 0 };
        let result = sqlx::query(
            r#"
            UPDATE user_extension_relation 
            SET open = ?
            WHERE user_uuid = ? and extension_uuid = ?
            "#,
        )
        .bind(open)
        .bind(user_uuid)
        .bind(extension_uuid)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() >= 1)
    }

    pub async fn environmnet_use_extension(
        pool: &Pool<Sqlite>,
        extension_uuid: &str,
        environment_uuid: &str,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let row = sqlx::query(
            "INSERT INTO environment_extension_relation (environment_uuid, extension_uuid) VALUES (?, ?)",
        )
        .bind(environment_uuid)
        .bind(extension_uuid)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(row.rows_affected() >= 1)
    }

    pub async fn environmnet_remove_extension(
        pool: &Pool<Sqlite>,
        extension_uuid: &str,
        environment_uuid: &str,
    ) -> Result<bool, Error> {
        let row = sqlx::query(
            "delete
                from environment_extension_relation
                where environment_uuid = ? and extension_uuid = ?;",
        )
        .bind(environment_uuid)
        .bind(extension_uuid)
        .execute(pool)
        .await?;
        Ok(row.rows_affected() >= 1)
    }

    pub async fn delete(pool: &Pool<Sqlite>, extension_uuid: &str) -> Result<bool, Error> {
        let result = sqlx::query(
            r#"
            UPDATE extensions
            SET deleted_at = CURRENT_TIMESTAMP
            WHERE uuid = ?
            "#,
        )
        .bind(extension_uuid)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() >= 1)
    }

    pub async fn remove_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        extension_uuid: &str,
    ) -> Result<bool, Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM user_extension_relation 
            WHERE user_uuid = ? AND extension_uuid = ?;
            "#,
        )
        .bind(user_uuid)
        .bind(extension_uuid)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() >= 1)
    }
}
