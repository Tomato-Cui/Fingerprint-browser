use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Environment {
    pub id: i32,                      // 自增ID
    pub uuid: Option<String>,         // UUID
    pub user_uuid: String,            // 用户UUID
    pub team_id: Option<i32>,         // 团队ID
    pub proxy_id: Option<i32>,        // 代理ID
    pub fp_info_id: Option<i32>,      // 指纹信息ID
    pub group_id: Option<i32>,        // 分组ID
    pub name: String,                 // 环境名称
    pub description: Option<String>,  // 环境描述
    pub default_urls: Option<String>, // 默认打开网页
    pub proxy_enable: i8,             // 代理启用
    pub created_at: Option<String>,   // 创建时间
    pub updated_at: Option<String>,   // 更新时间
    pub lasted_at: Option<String>,    // 最近时间
    pub deleted_at: Option<String>,   // 删除时间
}

impl Environment {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, environment: &Environment) -> Result<bool, Error> {
        let sql = r#"
        INSERT INTO environments (
            uuid, user_uuid, team_id, proxy_id, fp_info_id, group_id, name, description, default_urls, 
            proxy_enable
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#;

        let row = sqlx::query(sql)
            .bind(&environment.uuid)
            .bind(&environment.user_uuid)
            .bind(environment.team_id)
            .bind(environment.proxy_id)
            .bind(environment.fp_info_id)
            .bind(environment.group_id)
            .bind(&environment.name)
            .bind(&environment.description)
            .bind(&environment.default_urls)
            .bind(environment.proxy_enable)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() >= 1)
    }

    #[allow(dead_code)]
    pub async fn query_by_uuid(pool: &Pool<Sqlite>, uuid: &str) -> Result<Environment, Error> {
        let environment: Environment =
            sqlx::query_as("SELECT * FROM environments WHERE uuid = ? AND deleted_at IS NULL")
                .bind(uuid)
                .fetch_one(pool)
                .await?;

        Ok(environment)
    }

    #[allow(dead_code)]
    pub async fn query_environments_by_group_id(
        pool: &Pool<Sqlite>,
        group_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Environment>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments WHERE group_id = ? AND deleted_at IS NULL",
        )
        .bind(group_id)
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let environments: Vec<Environment> = sqlx::query_as(
            "SELECT * FROM environments WHERE group_id = ? AND deleted_at IS NULL LIMIT ? OFFSET ?",
        )
        .bind(group_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, environments))
    }

    #[allow(dead_code)]
    pub async fn query_environments_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Environment>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments WHERE user_uuid = ? AND deleted_at IS NULL",
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

        let environments: Vec<Environment> = sqlx::query_as(
        "SELECT * FROM environments WHERE user_uuid = ? AND deleted_at IS NULL LIMIT ? OFFSET ?",
        )
        .bind(user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, environments))
    }

    #[allow(dead_code)]
    pub async fn modify_basic_info(
        pool: &Pool<Sqlite>,
        uuid: &str,
        name: &str,
        description: Option<String>,
    ) -> Result<bool, Error> {
        let sql = "
            UPDATE environments
            SET name = ?, description = ?, updated_at = CURRENT_TIMESTAMP
            WHERE uuid = ? AND deleted_at IS NULL
        ";

        let row = sqlx::query(sql)
            .bind(name)
            .bind(description)
            .bind(uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_environments_by_team_id_and_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: i32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Environment>), Error> {
        let relation_exists: (i64,) = sqlx::query_as(
            "SELECT count(1) FROM user_team_relation 
         WHERE user_uuid = ? AND team_id = ? AND blocked = 0 AND deleted_at IS NULL",
        )
        .bind(user_uuid)
        .bind(team_id)
        .fetch_one(pool)
        .await?;

        if relation_exists.0 == 0 {
            return Err(Error::RowNotFound);
        }

        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments WHERE team_id = ? AND deleted_at IS NULL",
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

        let environments: Vec<Environment> = sqlx::query_as(
            "SELECT * FROM environments WHERE team_id = ? AND deleted_at IS NULL LIMIT ? OFFSET ?",
        )
        .bind(team_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, environments))
    }

    #[allow(dead_code)]
    pub async fn move_environment_to_group(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
        group_id: u32,
    ) -> Result<bool, Error> {
        let row = sqlx::query(
            r#"
            UPDATE environments
            SET
                group_id = ?,
                updated_at = DATETIME('now')
            WHERE uuid = ?;
            "#,
        )
        .bind(group_id)
        .bind(environment_uuid)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn delete_and_move_to_trash(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
        from_user_uuid: &str,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        sqlx::query(
            "INSERT INTO environment_trash (environment_uuid, from_user_uuid) VALUES (?, ?)",
        )
        .bind(environment_uuid)
        .bind(from_user_uuid)
        .execute(&mut *tx)
        .await?;

        let row = sqlx::query(
            "UPDATE environments SET deleted_at = DATETIME('now') WHERE uuid = ? and user_uuid = ?",
        )
        .bind(environment_uuid)
        .bind(from_user_uuid)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }
}
