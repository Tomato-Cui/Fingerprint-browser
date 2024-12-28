use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct TeamGroup {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub team_id: Option<i32>,
    pub team_group_permission_id: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
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
    ) -> Result<Value, Error> {
        let query_sql = r#"
            SELECT COUNT(*) as count, tg.*
            FROM user_team_relation utr
                    join team_groups tg on utr.team_group_id = tg.id
            WHERE utr.team_id = ?
            and utr.blocked = 0
            and is_leader = 0
            AND utr.deleted_at IS NULL
            GROUP BY utr.team_group_id;
        "#;

        let team_groups: Vec<crate::dto::team_group::TeamGroupList> = sqlx::query_as(&query_sql)
            .bind(team_id)
            .fetch_all(pool)
            .await?;

        let result_value = commons::util::struct_to_json_value(team_groups);

        Ok(result_value)
    }

    #[allow(dead_code)]
    pub async fn query_all_team_groups(
        pool: &Pool<Sqlite>,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<TeamGroup>), Error> {
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
