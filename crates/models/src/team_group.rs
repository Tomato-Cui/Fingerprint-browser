use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct TeamGroup {
    pub id: i32,                               // 自增ID
    pub name: String,                          // 组名称
    pub description: Option<String>,           // 组描述
    pub team_id: Option<i32>,                  // 团队ID
    pub team_group_permission_id: Option<i32>, // 团队组权限ID
    pub created_at: Option<String>,            // 创建时间
    pub updated_at: Option<String>,            // 更新时间
    pub deleted_at: Option<String>,            // 删除时间
}

impl TeamGroup {
    #[allow(dead_code)]
    pub async fn insert_team_group(
        pool: &Pool<Sqlite>,
        team_group: &TeamGroup,
    ) -> Result<bool, Error> {
        let sql = "
            INSERT INTO team_groups (
                name, description, team_id, team_group_permission_id
            ) VALUES (
                ?, ?, ?, ?
            )";

        let row = sqlx::query(sql)
            .bind(&team_group.name)
            .bind(&team_group.description)
            .bind(&team_group.team_id)
            .bind(&team_group.team_group_permission_id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_team_group_by_id(pool: &Pool<Sqlite>, id: u32) -> Result<TeamGroup, Error> {
        let team_group: TeamGroup =
            sqlx::query_as("SELECT * FROM team_groups WHERE id = ? AND deleted_at IS NULL")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(team_group)
    }

    #[allow(dead_code)]
    pub async fn query_team_group_by_team_id(
        pool: &Pool<Sqlite>,
        team_id: u32,
    ) -> Result<Vec<TeamGroup>, Error> {
        let team_groups: Vec<TeamGroup> =
            sqlx::query_as("SELECT * FROM team_groups WHERE team_id = ? AND deleted_at IS NULL")
                .bind(team_id)
                .fetch_all(pool)
                .await?;

        Ok(team_groups)
    }

    #[allow(dead_code)]
    pub async fn query_all_team_groups(
        pool: &Pool<Sqlite>,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<TeamGroup>), Error> {
        // 获取总数
        let (total,): (i64,) =
            sqlx::query_as("SELECT count(1) FROM team_groups WHERE deleted_at IS NULL")
                .fetch_one(pool)
                .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        // 获取分页的组列表
        let team_groups: Vec<TeamGroup> =
            sqlx::query_as("SELECT * FROM team_groups WHERE deleted_at IS NULL LIMIT ? OFFSET ?")
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?;

        Ok((total, team_groups))
    }

    #[allow(dead_code)]
    pub async fn update_team_group(
        pool: &Pool<Sqlite>,
        team_group: &TeamGroup,
    ) -> Result<bool, Error> {
        let sql = "
            UPDATE team_groups
            SET name = ?, description = ?, team_id = ?, team_group_permission_id = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND deleted_at IS NULL
        ";

        let row = sqlx::query(sql)
            .bind(&team_group.name)
            .bind(&team_group.description)
            .bind(&team_group.team_id)
            .bind(&team_group.team_group_permission_id)
            .bind(team_group.id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete_team_group(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let sql = "UPDATE team_groups SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";

        let row = sqlx::query(sql).bind(id).execute(pool).await?;

        Ok(row.rows_affected() == 1)
    }
}
