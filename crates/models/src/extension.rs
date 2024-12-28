use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Default)]
pub struct Extension {
    pub id: Option<i32>,
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub release_url: Option<String>,
    pub location_url: String,
    pub size: Option<i32>,
    pub all_can_use: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl Extension {
    pub async fn insert(
        pool: &Pool<Sqlite>,
        current_table: &str,
        current_id: &str,
        extension: Extension,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;
        sqlx::query(
            r#"
            INSERT INTO extensions (uuid, name, description, cover_url, release_url, location_url, size, all_can_use)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&extension.uuid)
        .bind(&extension.name)
        .bind(&extension.description)
        .bind(&extension.cover_url)
        .bind(&extension.release_url)
        .bind(&extension.location_url)
        .bind(&extension.size)
        .bind(&extension.all_can_use)
        .execute(&mut *tx)
        .await?;

        match current_table {
            "user_extension_relation" => {
                let row = sqlx::query("INSERT INTO user_extension_relation (user_uuid , extension_uuid) VALUES (?, ?) RETURNING id")
                    .bind(current_id)
                    .bind(extension.uuid)
                    .execute(&mut *tx)
                    .await?;
                Ok(row.rows_affected() >= 1)
            }
            "team_extension_relation" => {
                let row = sqlx::query("INSERT INTO team_extension_relation (team_id, extension_uuid) VALUES (?, ?) RETURNING id")
                    .bind(current_id)
                    .bind(extension.uuid)
                    .execute(&mut *tx)
                    .await?;
                Ok(row.rows_affected() >= 1)
            }
            _ => Ok(false),
        }
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
    ) -> Result<(i64, Vec<Extension>), Error> {
        let total: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM user_extension_relation ure WHERE uer.user_uuid = ? and deleted_at is null",
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

        let result: Vec<Extension> = sqlx::query_as(
            r#"
            SELECT e.* FROM extensions e 
                join user_extension_relation uer 
                    on e.uuid = uer.extension_uuid  
                WHERE uer.user_uuid = ? and deleted_at is null
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
                WHERE eer.environment_uuid = ? and deleted_at is null
                LIMIT ? OFFSET ?
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
            SET name = ?, description = ?, cover_url = ?, release_url = ?, location_url = ?, size = ?, updated_at = CURRENT_TIMESTAMP
            WHERE uuid = ?
            "#
        )
        .bind(&extension.name)
        .bind(&extension.description)
        .bind(&extension.cover_url)
        .bind(&extension.release_url)
        .bind(&extension.location_url)
        .bind(&extension.size)
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
        let row = sqlx::query(
            "INSERT INTO user_extension_relation (environmnet_uuid, extension_uuid) VALUES (?, ?)",
        )
        .bind(extension_uuid)
        .bind(environment_uuid)
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
}
