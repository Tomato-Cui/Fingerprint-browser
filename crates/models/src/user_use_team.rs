use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct UserUseTeam {
    pub id: i32,
    pub user_uuid: String,
    pub team_id: u32,
    pub created_at: Option<String>, // 创建时间
    pub updated_at: Option<String>, // 更新时间
    pub deleted_at: Option<String>, // 删除时间
}

impl UserUseTeam {
    pub async fn create(
        pool: &Pool<Sqlite>,
        user_uuid: &String,
        team_id: u32,
    ) -> Result<bool, Error> {
        let result = match UserUseTeam::query_current_team_id_by_user_uuid(pool, &user_uuid).await {
            Ok(team_id) => UserUseTeam::update(pool, &user_uuid, team_id).await?,
            Err(e) => match e {
                Error::RowNotFound => {
                    let row = sqlx::query(
                        r#"
                        INSERT INTO user_use_team (user_uuid, team_id)
                        VALUES (?, ?)
                        "#,
                    )
                    .bind(user_uuid)
                    .bind(team_id)
                    .execute(pool)
                    .await?;
                    row.rows_affected() >= 1
                }
                _ => false,
            },
        };

        Ok(result)
    }

    pub async fn query_current_team_id_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
    ) -> Result<u32, Error> {
        let team_id: u32 = sqlx::query_scalar(
            r#"
            SELECT team_id
            FROM user_use_team
            WHERE user_uuid = ?
            "#,
        )
        .bind(user_uuid)
        .fetch_one(pool)
        .await?;

        Ok(team_id)
    }

    pub async fn update(pool: &Pool<Sqlite>, user_uuid: &str, team_id: u32) -> Result<bool, Error> {
        let row = sqlx::query(
            r#"
            UPDATE user_use_team
            SET team_id = ?
            WHERE user_uuid = ?
            "#,
        )
        .bind(user_uuid)
        .bind(team_id)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() >= 1)
    }
}
