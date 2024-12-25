use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct UserTeamTemp {
    pub id: Option<i32>,             // 自增ID
    pub user_uuid: String,           // 用户UUID
    pub team_id: String,             // 团队ID
    pub allow_1: Option<i32>,        // 允许1
    pub allow_2: Option<i32>,        // 允许2
    pub description: Option<String>, // 描述
    pub created_at: Option<String>,  // 创建时间
    pub updated_at: Option<String>,  // 更新时间
    pub deleted_at: Option<String>,  // 删除时间
}

impl UserTeamTemp {
    #[allow(dead_code)]
    pub async fn insert_user_team_temp(
        pool: &Pool<Sqlite>,
        user_team_temp: &UserTeamTemp,
    ) -> Result<bool, Error> {
        let sql = "
            INSERT INTO user_team_temps (
                user_uuid, team_id, allow_1, allow_2, description
            ) VALUES (
                ?, ?, ?, ?, ?
            )";

        let row = sqlx::query(sql)
            .bind(&user_team_temp.user_uuid)
            .bind(&user_team_temp.team_id)
            .bind(&user_team_temp.allow_1)
            .bind(&user_team_temp.allow_2)
            .bind(&user_team_temp.description)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_user_team_temp_by_id(
        pool: &Pool<Sqlite>,
        id: i32,
    ) -> Result<UserTeamTemp, Error> {
        let user_team_temp: UserTeamTemp =
            sqlx::query_as("SELECT * FROM user_team_temps WHERE id = ? AND deleted_at IS NULL")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(user_team_temp)
    }

    #[allow(dead_code)]
    pub async fn query_all_user_team_temps(
        pool: &Pool<Sqlite>,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<UserTeamTemp>), Error> {
        // 获取总数
        let (total,): (i64,) =
            sqlx::query_as("SELECT count(1) FROM user_team_temps WHERE deleted_at IS NULL")
                .fetch_one(pool)
                .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        // 获取分页的用户团队临时关系列表
        let user_team_temps: Vec<UserTeamTemp> = sqlx::query_as(
            "SELECT * FROM user_team_temps WHERE deleted_at IS NULL LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, user_team_temps))
    }

    #[allow(dead_code)]
    pub async fn update_user_team_temp(
        pool: &Pool<Sqlite>,
        user_team_temp: &UserTeamTemp,
    ) -> Result<bool, Error> {
        let sql = "
            UPDATE user_team_temps
            SET user_uuid = ?, team_id = ?, allow_1 = ?, allow_2 = ?, description = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND deleted_at IS NULL
        ";

        let row = sqlx::query(sql)
            .bind(&user_team_temp.user_uuid)
            .bind(&user_team_temp.team_id)
            .bind(&user_team_temp.allow_1)
            .bind(&user_team_temp.allow_2)
            .bind(&user_team_temp.description)
            .bind(user_team_temp.id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete_user_team_temp(pool: &Pool<Sqlite>, id: i32) -> Result<bool, Error> {
        let sql = "UPDATE user_team_temps SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";

        let row = sqlx::query(sql).bind(id).execute(pool).await?;

        Ok(row.rows_affected() == 1)
    }
}
