use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Codes {
    pub id: u32,
    pub kind: String,
    pub key: String,
    pub value: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
    pub expired_at: Option<String>,
}

impl Codes {
    pub async fn insert(
        pool: &Pool<Sqlite>,
        kind: &str,
        key: &str,
        value: &str,
        expired_at: &str,
    ) -> Result<bool, Error> {
        let row = sqlx::query(
                "INSERT INTO codes (kind, key, value, expired_at) 
                VALUES (?, ?, ?, ?)",
            )
            .bind(kind)
            .bind(key)
            .bind(value)
            .bind(expired_at)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() >= 1)
    }

    pub async fn update(
        pool: &Pool<Sqlite>,
        id: u32,
        key: &str,
        value: &str,
        expired_at: &str,
    ) -> Result<bool, Error> {
        let row = 
            sqlx::query("update codes set updated_at = DATETIME('now'), expired_at = ?, key = ?,value = ? where id = ?")
            .bind(expired_at)
            .bind(key)
            .bind(value)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() > 0)
    }

    // 删除记录
    pub async fn delete(pool: &Pool<Sqlite>, id: u32) -> Result<bool, Error> {
        let row = sqlx::query("UPDATE codes SET deleted_at = DATETIME('now') WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    pub async fn is_ok(pool: &Pool<Sqlite>,
        kind: &str,
        key: &str,
        value: &str) -> Result<(bool, String), Error> {
            let existing_record = sqlx::query_as::<_, Codes>(
                "SELECT * FROM codes WHERE kind = ? AND key = ? AND deleted_at IS NULL ORDER BY id DESC limit 1"
            )
                .bind(kind)
                .bind(key)
                .bind(value)
                .bind(commons::time::get_system_time_mills() as i32)
                .fetch_optional(pool)
                .await?;

            Ok(if let Some(code) = existing_record{
                if code.value != value{
                    (false, "验证码错误".to_string())
                }else{
                    let expired_at:u128 = code.expired_at.unwrap_or_default().parse().unwrap_or_default();
                    if expired_at > commons::time::get_system_time_mills(){
                        (true, "".to_string())
                    }else{
                        (false, "验证码已过期".to_string())
                    }
                }
            }else{
                (false, "请发送邮箱验证码".to_string())
            })
    }
}