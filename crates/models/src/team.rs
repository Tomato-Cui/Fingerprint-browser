use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Team {
    pub id: i32,                     // 自增ID
    pub name: String,                // 团队名称
    pub description: Option<String>, // 团队描述
    pub created_at: Option<String>,  // 创建时间
    pub updated_at: Option<String>,  // 更新时间
    pub deleted_at: Option<String>,  // 删除时间
}

impl Team {
    #[allow(dead_code)]
    pub async fn insert(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team: &Team,
        description: Option<String>,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let sql = "
            INSERT INTO teams (
                name, description
            ) VALUES (
                ?, ?
            ) RETURNING id";

        let team_id: i32 = sqlx::query_scalar(sql)
            .bind(&team.name)
            .bind(&team.description)
            .fetch_one(&mut *tx)
            .await?;

        let default_groups = vec![
            ("管理组", "管理组描述", 4), 
            ("编辑组", "编辑组描述", 2), 
            ("权限组", "权限组描述", 3), 
            ("默认组", "默认组描述", 1), 
        ];

        let mut group_ids = Vec::new();
        for (name, description, permission_id) in default_groups {
            let sql = "
            INSERT INTO team_groups (
                name, description, team_id, team_group_permission_id
            ) VALUES (
                ?, ?, ?, ?
            ) RETURNING id";

            let group_id: i32 = sqlx::query_scalar(sql)
                .bind(name)
                .bind(description)
                .bind(team_id)
                .bind(permission_id)
                .fetch_one(&mut *tx)
                .await?;

            group_ids.push(group_id);
        }

        let sql = "
            INSERT INTO user_team_relation (
                user_uuid, team_id, team_group_id, is_leader, blocked, description
            ) VALUES (
                ?, ?, NULL, 1, 0, ?
            )";

        let row = sqlx::query(sql)
            .bind(user_uuid)
            .bind(team_id)
            .bind(description)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_team_by_id(pool: &Pool<Sqlite>, id: u32) -> Result<Team, Error> {
        let team: Team = sqlx::query_as("SELECT * FROM teams WHERE id = ? AND deleted_at IS NULL")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(team)
    }

    #[allow(dead_code)]
    pub async fn query_team_by_id_and_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: i32,
    ) -> Result<Team, Error> {
        let team: Team = sqlx::query_as(
            "SELECT t.* FROM teams t
         JOIN user_team_relation utr ON t.id = utr.team_id
         WHERE t.id = ? AND utr.user_uuid = ? AND t.deleted_at IS NULL AND utr.deleted_at IS NULL",
        )
        .bind(team_id)
        .bind(user_uuid)
        .fetch_one(pool)
        .await?;

        Ok(team)
    }

    #[allow(dead_code)]
    pub async fn query_teams_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Team>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM teams t
         JOIN user_team_relation utr ON t.id = utr.team_id
         WHERE utr.user_uuid = ? AND t.deleted_at IS NULL AND utr.deleted_at IS NULL",
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

        let teams: Vec<Team> = sqlx::query_as(
            "SELECT t.* FROM teams t
         JOIN user_team_relation utr ON t.id = utr.team_id
         WHERE utr.user_uuid = ? AND t.deleted_at IS NULL AND utr.deleted_at IS NULL
         LIMIT ? OFFSET ?",
        )
        .bind(user_uuid)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, teams))
    }

    #[allow(dead_code)]
    pub async fn update(pool: &Pool<Sqlite>, id: u32, team: &Team) -> Result<bool, Error> {
        let sql = "
        UPDATE teams
            SET name = ?, description = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND deleted_at IS NULL;
        ";

        let row = sqlx::query(sql)
            .bind(&team.name)
            .bind(&team.description)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let sql = "UPDATE teams SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";

        let row = sqlx::query(sql).bind(id).execute(pool).await?;

        Ok(row.rows_affected() == 1)
    }
}
