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
    pub async fn query_team_by_name(pool: &Pool<Sqlite>, name: &str) -> Result<Team, Error> {
        let team: Team =
            sqlx::query_as("SELECT * FROM teams WHERE name = ? AND deleted_at IS NULL")
                .bind(name)
                .fetch_one(pool)
                .await?;

        Ok(team)
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
    pub async fn query_default_team_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
    ) -> Result<Team, Error> {
        let team: Team = sqlx::query_as(
            "SELECT t.* FROM teams t
             JOIN user_team_relation utr ON t.id = utr.team_id
             WHERE utr.user_uuid = ? AND t.deleted_at IS NULL AND utr.deleted_at IS NULL
             ORDER BY utr.created_at ASC
             LIMIT 1",
        )
        .bind(user_uuid)
        .fetch_one(pool)
        .await?;

        Ok(team)
    }

    #[allow(dead_code)]
    pub async fn query_all_team_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
    ) -> Result<Vec<Team>, Error> {
        let team_ids: Vec<i32> = sqlx::query_scalar(
            "SELECT DISTINCT team_id FROM user_team_relation WHERE user_uuid = ? AND deleted_at IS NULL"
        )
        .bind(user_uuid)
        .fetch_all(pool)
        .await?;

        let teams: Vec<Team> =
            sqlx::query_as("SELECT * FROM teams WHERE id IN (?) AND deleted_at IS NULL")
                .bind(
                    team_ids
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                )
                .fetch_all(pool)
                .await?;

        Ok(teams)
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
    pub async fn query_team_all_user_by_group_id(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: u32,
        group_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<crate::dto::team_info::UserInfoWithGroup>), Error> {
        let (is_leader,): (i64,) =
            sqlx::query_as("SELECT count(1) FROM user_team_relation WHERE team_id = ? and user_uuid = ? and is_leader = 1")
                .bind(team_id)
                .bind(user_uuid)
                .fetch_one(pool)
                .await?;

        if !(is_leader > 0) {
            return Ok((0, vec![]));
        }

        let user_uuids: Vec<String> =
            sqlx::query_scalar("SELECT user_uuid FROM user_team_relation WHERE team_group_id = ? and is_leader = 0 and blocked = 0 and deleted_at is null")
                .bind(group_id)
                .fetch_all(pool)
                .await?;

        let total = user_uuids.len() as i64;
        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;
        let user_uuid_str = user_uuids
            .iter()
            .map(|v| format!("'{}'", v))
            .collect::<Vec<String>>()
            .join(",");
        let select_sql = format!(
            "
            SELECT 
                users.uuid as user_uuid,
                user_infos.nickname,
                user_infos.email,
                user_team_relation.team_id, 
                user_team_relation.team_group_id, 
                user_team_relation.description, 
                team_groups.name AS group_name 
            FROM 
                user_infos 
            JOIN 
                users ON user_infos.id = users.user_info_id 
            JOIN 
                user_team_relation ON users.uuid = user_team_relation.user_uuid 
            LEFT JOIN 
                team_groups ON user_team_relation.team_group_id = team_groups.id 
            WHERE 
                user_team_relation.team_group_id = ?
                and user_team_relation.user_uuid in ({})
                AND user_team_relation.is_leader = 0 
                AND user_team_relation.blocked = 0 
                and user_team_relation.deleted_at is null
            LIMIT ? OFFSET ?
        ",
            user_uuid_str
        );

        let user_infos: Vec<crate::dto::team_info::UserInfoWithGroup> = sqlx::query_as(&select_sql)
            .bind(group_id)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok((total, user_infos))
    }

    #[allow(dead_code)]
    pub async fn query_team_all_user_by_team_id(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: u32,
        blocked: bool,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<crate::dto::team_info::UserInfoWithGroup>), Error> {
        let (is_leader,): (i64,) =
            sqlx::query_as("SELECT count(1) FROM user_team_relation WHERE team_id = ? and user_uuid = ? and is_leader = 1")
                .bind(team_id)
                .bind(user_uuid)
                .fetch_one(pool)
                .await?;

        if !(is_leader > 0) {
            return Ok((0, vec![]));
        }
        let blocked = if blocked { 1 } else { 0 };

        let (total,): (i64,) =
            sqlx::query_as("SELECT count(1) FROM user_team_relation WHERE team_id = ? and is_leader = 0 and blocked = ? and deleted_at is null")
                .bind(team_id)
                .bind(blocked)
                .fetch_one(pool)
                .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let query_sql = "
            SELECT 
                users.uuid as user_uuid,
                user_infos.nickname,
                user_infos.email,
                user_team_relation.team_group_id, 
                user_team_relation.team_id, 
                user_team_relation.description, 
                team_groups.name AS group_name 
            FROM 
                user_infos 
            JOIN 
                users ON user_infos.id = users.user_info_id 
            JOIN 
                user_team_relation ON users.uuid = user_team_relation.user_uuid 
            LEFT JOIN 
                team_groups ON user_team_relation.team_group_id = team_groups.id 
            WHERE 
                user_team_relation.team_id = ? 
                AND user_team_relation.is_leader = 0 
                AND user_team_relation.blocked = ? 
                and user_team_relation.deleted_at is null
            LIMIT ? OFFSET ?
        ";

        let user_infos: Vec<crate::dto::team_info::UserInfoWithGroup> = sqlx::query_as(&query_sql)
            .bind(team_id)
            .bind(blocked)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok((total, user_infos))
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
    pub async fn blocked_action(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: u32,
        current_user_uuid: &str,
        blocked: bool,
    ) -> Result<bool, Error> {
        let blocked = if blocked { 1 } else { 0 };
        let user_uuids: Vec<String> = sqlx::query_scalar(
        "select user_uuid from user_team_relation where user_uuid = ? and team_id = ? and is_leader = 1"
            )
            .bind(user_uuid)
            .bind(team_id)
            .fetch_all(pool)
            .await?;
        if user_uuids.is_empty() {
            return Ok(false);
        }

        let sql = "
            UPDATE user_team_relation
                SET blocked = ?, updated_at = CURRENT_TIMESTAMP
            WHERE user_uuid = ? AND team_id = ? AND deleted_at IS NULL;
            ";

        let row = sqlx::query(sql)
            .bind(blocked)
            .bind(current_user_uuid)
            .bind(team_id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn remove_user(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: u32,
        current_user_uuid: &str,
    ) -> Result<bool, Error> {
        let user_uuids: Vec<String> = sqlx::query_scalar(
        "select user_uuid from user_team_relation where user_uuid = ? and team_id = ? and is_leader = 1"
            )
            .bind(user_uuid)
            .bind(team_id)
            .fetch_all(pool)
            .await?;
        if user_uuids.is_empty() {
            return Ok(false);
        }

        let sql = "
            UPDATE user_team_relation
                SET deleted_at = CURRENT_TIMESTAMP
            WHERE user_uuid = ? AND team_id = ? AND deleted_at IS NULL;
            ";

        let row = sqlx::query(sql)
            .bind(current_user_uuid)
            .bind(team_id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn switch_team(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: u32,
    ) -> Result<bool, Error> {
        let sql = "
            UPDATE user_use_team
                set team_id = ?
            WHERE user_uuid = ? AND deleted_at IS NULL;
            ";

        let row = sqlx::query(sql)
            .bind(team_id)
            .bind(user_uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    pub async fn update_team_user_info(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: u32,
        description: Option<String>,
        team_group_id: u32,
        current_user_uuid: &str,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let user_uuids: Vec<String> = sqlx::query_scalar(
        "select user_uuid from user_team_relation where user_uuid = ? and team_id = ? and is_leader = 1"
            )
            .bind(user_uuid)
            .bind(team_id)
            .fetch_all(pool)
            .await?;
        if user_uuids.is_empty() {
            return Ok(false);
        }

        let sql = "
            UPDATE user_team_relation
            SET description = ?, team_group_id = ?, updated_at = CURRENT_TIMESTAMP
            WHERE user_uuid = ? AND team_id = ? and is_leader = 0 and deleted_at IS NULL;
        ";

        sqlx::query(sql)
            .bind(description)
            .bind(team_group_id)
            .bind(current_user_uuid)
            .bind(team_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(true)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let sql = "UPDATE teams SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";

        let row = sqlx::query(sql).bind(id).execute(pool).await?;

        Ok(row.rows_affected() == 1)
    }
}
