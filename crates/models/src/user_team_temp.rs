use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

use crate::dto;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct UserTeamTemp {
    pub id: Option<i32>,             // 自增ID
    pub user_uuid: String,           // 用户UUID 1
    pub team_id: u32,                // team_id
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
    pub async fn query_user_apply(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<dto::user_team_temp::UserTeamTempInfo>), Error> {
        let mut tx = pool.begin().await?;

        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM user_team_temps 
         WHERE user_uuid = ? AND allow_2 = 1 AND deleted_at IS NULL",
        )
        .bind(user_uuid)
        .fetch_one(&mut *tx)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let query_sql = "
            SELECT utt.id, utt.allow_1, utt.allow_2, u.uuid as user_uuid, ui.nickname, ui.email, utt.description, utt.created_at, utt.updated_at, utt.deleted_at
            FROM user_team_temps utt
            JOIN users u ON utt.user_uuid = u.uuid
            JOIN user_infos ui ON u.user_info_id = ui.id
            WHERE utt.user_uuid = ? AND utt.allow_2 = 1 AND utt.deleted_at IS NULL 
            LIMIT ? OFFSET ?
        ";

        let user_team_temps: Vec<dto::user_team_temp::UserTeamTempInfo> = sqlx::query_as(query_sql)
            .bind(user_uuid)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok((total, user_team_temps))
    }

    pub async fn query_team_apply(
        pool: &Pool<Sqlite>,
        team_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<dto::user_team_temp::UserTeamTempInfo>), Error> {
        let mut tx = pool.begin().await?;

        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM user_team_temps 
         WHERE team_id = ?",
        )
        .bind(team_id)
        .fetch_one(&mut *tx)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let query_sql = "
            SELECT utt.id, utt.allow_1, utt.allow_2, u.uuid as user_uuid, ui.nickname, ui.email, utt.description, utt.created_at, utt.updated_at, utt.deleted_at
            FROM user_team_temps utt
            JOIN users u ON utt.user_uuid = u.uuid
            JOIN user_infos ui ON u.user_info_id = ui.id
            WHERE utt.team_id = ? AND utt.allow_2 = 0
            LIMIT ? OFFSET ?
        ";

        let user_team_temps: Vec<dto::user_team_temp::UserTeamTempInfo> = sqlx::query_as(query_sql)
            .bind(team_id)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok((total, user_team_temps))
    }

    #[allow(dead_code)]
    pub async fn agree_join(
        pool: &Pool<Sqlite>,
        id: u32,
        user_team_temp: &UserTeamTemp,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;
        let sql = "
            UPDATE user_team_temps
            SET user_uuid = ?, team_id = ?, allow_1 = ?, allow_2 = ?, description = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        ";

        sqlx::query(sql)
            .bind(&user_team_temp.user_uuid)
            .bind(&user_team_temp.team_id)
            .bind(&user_team_temp.allow_1)
            .bind(&user_team_temp.allow_2)
            .bind(&user_team_temp.description)
            .bind(id)
            .execute(&mut *tx)
            .await?;

        let sql = "
            INSERT INTO user_team_relation (
            user_uuid, team_id, team_group_id, is_leader, blocked
            ) VALUES (
            ?, ?, (SELECT id FROM team_groups WHERE team_id = ? AND deleted_at IS NULL ORDER BY id DESC LIMIT 1), 0, 0
            )";

        let row = sqlx::query(sql)
            .bind(&user_team_temp.user_uuid)
            .bind(&user_team_temp.team_id)
            .bind(&user_team_temp.team_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn receive_user_to_team(
        pool: &Pool<Sqlite>,
        id: u32,
        user_uuid: &str,
        team_id: u32,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let sql = "
            UPDATE user_team_temps
            SET allow_1 = 1, allow_2 = 1, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND deleted_at IS NULL
        ";
        let row = sqlx::query(sql).bind(id).execute(&mut *tx).await?;
        if row.rows_affected() > 0 {
            return Err(Error::RowNotFound);
        }

        let team_group_id: i32 = sqlx::query_scalar(
            "SELECT id FROM team_groups WHERE team_id = ? AND deleted_at IS NULL LIMIT 1",
        )
        .bind(team_id)
        .fetch_one(&mut *tx)
        .await?;

        let sql = "
        INSERT INTO user_team_relation (
            user_uuid, team_id, team_group_id, is_leader, blocked
        ) VALUES (
            ?, ?, ?, 0, 0
        )";

        let row = sqlx::query(sql)
            .bind(user_uuid)
            .bind(team_id)
            .bind(team_group_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete_user_team_temp(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let sql = "UPDATE user_team_temps SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";

        let row = sqlx::query(sql).bind(id).execute(pool).await?;

        Ok(row.rows_affected() == 1)
    }
}
