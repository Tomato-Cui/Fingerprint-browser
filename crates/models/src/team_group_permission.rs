use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct TeamGroupPermission {
    pub id: Option<i32>,             // 自增ID
    pub name: String,                // 权限名称
    pub description: Option<String>, // 权限描述
    pub action: Option<String>,      // 权限描述
    pub created_at: Option<String>,  // 创建时间
    pub updated_at: Option<String>,  // 更新时间
    pub deleted_at: Option<String>,  // 删除时间
}

impl TeamGroupPermission {
    #[allow(dead_code)]
    pub async fn check_permission(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        action: &str,
    ) -> Result<bool, Error> {
        let (is_leader,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM user_team_relation
         WHERE user_uuid = ? AND is_leader = 1 AND deleted_at IS NULL",
        )
        .bind(user_uuid)
        .fetch_one(pool)
        .await?;

        if is_leader > 0 {
            return Ok(true);
        }

        let (team_group_permission_id,): (i32,) = sqlx::query_as(
            "SELECT id FROM team_group_permission
         WHERE action = ? AND deleted_at IS NULL",
        )
        .bind(action)
        .fetch_one(pool)
        .await?;

        let (exists,): (i64,) = sqlx::query_as(
        "SELECT count(1) FROM user_team_relation utr
            JOIN team_groups tg ON utr.team_group_id = tg.id
            WHERE utr.user_uuid = ? AND tg.team_group_permission_id = ? AND utr.deleted_at IS NULL AND tg.deleted_at IS NULL"
        )
        .bind(user_uuid)
        .bind(team_group_permission_id)
        .fetch_one(pool)
        .await?;

        Ok(exists > 0)
    }

    #[allow(dead_code)]
    pub async fn grant_permission(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: i32,
        action: &str,
    ) -> Result<bool, Error> {
        let (team_group_permission_id,): (i32,) = sqlx::query_as(
            "SELECT id FROM team_group_permission
         WHERE action = ? AND deleted_at IS NULL",
        )
        .bind(action)
        .fetch_one(pool)
        .await?;

        let (team_group_id,): (i32,) = sqlx::query_as(
            "SELECT id FROM team_groups
         WHERE team_id = ? AND team_group_permission_id = ? AND deleted_at IS NULL",
        )
        .bind(team_id)
        .bind(team_group_permission_id)
        .fetch_one(pool)
        .await?;

        let sql = "
            INSERT INTO user_team_relation (
                user_uuid, team_id, team_group_id, is_leader, blocked, description
            ) VALUES (
                ?, ?, ?, 0, 0, ?
            )";

        let row = sqlx::query(sql)
            .bind(user_uuid)
            .bind(team_id)
            .bind(team_group_id)
            .bind("Granted permission")
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
