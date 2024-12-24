use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl Team {
    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, user_id: u32, team: &Team) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let (team_id,): (u32,) =
            sqlx::query_as("insert into teams (name, description) values(?, ?) returning id;")
                .bind(&team.name)
                .bind(&team.description)
                .fetch_one(&mut *tx)
                .await?;

        sqlx::query("insert into user_team_relation (user_id, team_id) values(?, ?)")
            .bind(&user_id)
            .bind(&team_id)
            .fetch_one(&mut *tx)
            .await?;

        Ok(team_id != 0)
    }

    #[allow(dead_code)]
    pub async fn query_id(pool: &Pool<Sqlite>, user_id: u32, id: u32) -> Result<Team, Error> {
        let (team_id,): (u32,) =
            sqlx::query_as("select id from user_team_relation where id = ? and user_id = ?")
                .bind(id)
                .bind(user_id)
                .fetch_one(pool)
                .await?;

        let team: Team = sqlx::query_as("select id from teams where id = ?")
            .bind(team_id)
            .fetch_one(pool)
            .await?;

        Ok(team)
    }

    #[allow(dead_code)]
    pub async fn query(
        pool: &Pool<Sqlite>,
        user_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Team>), Error> {
        let team_ids: Vec<(u32,)> =
            sqlx::query_as("select team_id from user_team_relation where user_id = ?")
                .bind(user_id)
                .fetch_all(pool)
                .await?;

        let total = team_ids.len() as i64;
        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        if total > 0 {
            let team_ids_str = team_ids
                .iter()
                .map(|(id,)| id.to_string())
                .collect::<Vec<String>>()
                .join(",");

            let team_id_sql = format!("({})", team_ids_str);
            let teams = sqlx::query_as("select * from teams where id in ? limit ? offset ?")
                .bind(team_id_sql)
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?;

            Ok((total as i64, teams))
        } else {
            Ok((total as i64, vec![]))
        }
    }

    #[allow(dead_code)]
    pub async fn update(pool: &Pool<Sqlite>, user_id: u32, team: &Team) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;
        let team_ids: Vec<(u32,)> =
            sqlx::query_as("select team_id from user_team_relation where user_id = ?")
                .bind(user_id)
                .fetch_all(&mut *tx)
                .await?;

        let ok = team_ids.iter().any(|(id,)| *id as i32 == team.id);

        if ok {
            sqlx::query(
                "update teams set name = ?, description = ?, updated_at = DATETIME('now') where id = ?",
            )
            .bind(&team.name)
            .bind(&team.description)
            .bind(team.id)
            .execute(&mut *tx)
            .await?;
        };

        Ok(true)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, user_id: u32, team_id: u32) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;
        let (team_id,): (u32,) = sqlx::query_as(
            "select team_id from user_team_relation where user_id = ? and team_id = ?",
        )
        .bind(user_id)
        .bind(team_id)
        .fetch_one(&mut *tx)
        .await?;

        let row = sqlx::query("delete from teams where id = ? ")
            .bind(team_id)
            .execute(&mut *tx)
            .await?;

        Ok(row.rows_affected() > 1)
    }
}
