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
    pub async fn can_check_permission(
        pool: &Pool<Sqlite>,
        resource_name: &str,
    ) -> Result<bool, Error> {
        let exists: Option<i32> =
            sqlx::query_scalar("SELECT 1 FROM allowed_operations WHERE resource_name = ?")
                .bind(resource_name)
                .fetch_optional(pool)
                .await?;
        eprintln!("{:?}", exists);

        Ok(exists.is_some_and(|v| v >= 1))
    }

    pub async fn check_permission(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        resource_name: &str,
    ) -> Result<bool, Error> {
        let team_id = if let Ok(team_id) =
            crate::user_use_team::UserUseTeam::query_current_team_id_by_user_uuid(pool, user_uuid)
                .await
        {
            team_id
        } else {
            return Ok(false);
        };

        let current_user_team_relation_info: Option<(i32, String, i32, i32, i32, String)> =
            sqlx::query_as(
                "SELECT id, user_uuid, team_id, team_group_id, is_leader, description
            FROM user_team_relation 
            WHERE user_uuid = ? AND team_id = ? and blocked != 1",
            )
            .bind(user_uuid)
            .bind(team_id)
            .fetch_optional(pool)
            .await?;

        let current_user_team_relation_info = match current_user_team_relation_info {
            Some(info) => info,
            None => {
                return Ok(false);
            }
        };

        // 3. 检查用户是否是团队的领导者
        let is_leader = current_user_team_relation_info.4;
        if is_leader == 1 {
            return Ok(true);
        }

        let team_group_id = current_user_team_relation_info.3; // team_group_id 字段

        let (exists,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM team_groups tg
         JOIN allowed_operations ao ON tg.team_group_permission_id = ao.operation_type
         WHERE tg.id = ? AND ao.resource_name = ?
         AND tg.deleted_at IS NULL AND",
        )
        .bind(team_group_id)
        .bind(resource_name)
        .fetch_one(pool)
        .await?;

        Ok(exists > 0)
    }

    #[allow(dead_code)]
    pub async fn grant_permission(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: u32,
        resource_name: &str,
    ) -> Result<bool, Error> {
        // 1. 查询 allowed_operations 表，获取 operation_type
        let operation_type: Option<String> = sqlx::query_scalar(
            "SELECT operation_type FROM allowed_operations 
         WHERE resource_name = ? AND deleted_at IS NULL",
        )
        .bind(resource_name)
        .fetch_optional(pool)
        .await?;

        let operation_type = match operation_type {
            Some(op_type) => op_type,
            None => {
                return Ok(false);
            }
        };

        // 2. 查询 team_group_permission 表，获取 team_group_permission_id
        let (team_group_permission_id,): (i32,) = sqlx::query_as(
            "SELECT id FROM team_group_permission
         WHERE action = ? AND deleted_at IS NULL",
        )
        .bind(operation_type)
        .fetch_one(pool)
        .await?;

        // 3. 查询 team_groups 表，获取 team_group_id
        let (team_group_id,): (i32,) = sqlx::query_as(
            "SELECT id FROM team_groups
         WHERE team_id = ? AND team_group_permission_id = ? AND deleted_at IS NULL",
        )
        .bind(team_id)
        .bind(team_group_permission_id)
        .fetch_one(pool)
        .await?;

        // 4. 插入用户与团队的关联信息
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
