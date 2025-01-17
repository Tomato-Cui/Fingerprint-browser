use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentTransferHistory {
    pub id: Option<i32>,            // 自增ID
    pub environment_uuid: String,   // 环境UUID
    pub from_user_uuid: String,     // 转移发起者UUID
    pub to_user_uuid: String,       // 接收者UUID
    pub created_at: Option<String>, // 创建时间
    pub updated_at: Option<String>, // 更新时间
    pub deleted_at: Option<String>, // 删除时间
}

impl EnvironmentTransferHistory {
    #[allow(dead_code)]
    pub async fn insert_transfer_history(
        pool: &Pool<Sqlite>,
        transfer_history: &EnvironmentTransferHistory,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let sql = "
        INSERT INTO environment_transfer_history (
            environment_uuid, from_user_uuid, to_user_uuid
        ) VALUES (
            ?, ?, ?
        )";

        let row = sqlx::query(sql)
            .bind(&transfer_history.environment_uuid)
            .bind(&transfer_history.from_user_uuid)
            .bind(&transfer_history.to_user_uuid)
            .execute(&mut *tx)
            .await?;

        let sql = "
        UPDATE environments
        SET user_uuid = ?, updated_at = CURRENT_TIMESTAMP
        WHERE uuid = ?";

        sqlx::query(sql)
            .bind(&transfer_history.to_user_uuid)
            .bind(&transfer_history.environment_uuid)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_environments_by_user_uuid(
        pool: &Pool<Sqlite>,
        from_user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Value>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments e
         JOIN environment_transfer_history eth ON e.uuid = eth.environment_uuid
         WHERE eth.from_user_uuid = ? AND e.deleted_at IS NULL AND eth.deleted_at IS NULL",
        )
        .bind(from_user_uuid)
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let query = "
        SELECT 
            e.id, e.uuid, e.user_uuid, e.team_id, e.proxy_id, e.fp_info_id, e.group_id, eg.name as group_name, e.name, e.description, e.default_urls, e.proxy_enable, e.created_at, e.updated_at, e.lasted_at, e.deleted_at,
            ef.id AS fp_id, ef.browser, ef.ua, ef.os, ef.country, ef.region, ef.city, ef.language_type, ef.languages, ef.gmt, ef.geography, ef.geo_tips, ef.geo_rule, ef.longitude, ef.latitude, ef.radius, ef.height, ef.width, ef.fonts_type, ef.fonts, ef.font_fingerprint, ef.web_rtc, ef.web_rtc_local_ip, ef.canvas, ef.webgl, ef.hardware_acceleration, ef.webgl_info, ef.audio_context, ef.speech_voices, ef.media, ef.cpu, ef.memory, ef.do_not_track, ef.battery, ef.port_scan, ef.white_list, ef.created_at AS fp_created_at, ef.updated_at AS fp_updated_at, ef.deleted_at AS fp_deleted_at,
            p.kind AS proxy_kind, p.host AS proxy_host, p.port AS proxy_port, p.username AS proxy_username, p.password AS proxy_password, p.user_uuid AS proxy_user_uuid, p.environment_group_id AS proxy_environment_group_id, p.created_at AS proxy_created_at, p.updated_at AS proxy_updated_at, p.deleted_at AS proxy_deleted_at
        FROM 
            environments e
        LEFT JOIN 
            environment_fingerprints ef ON e.fp_info_id = ef.id
        LEFT JOIN 
            environment_groups eg ON e.group_id = eg.id
        LEFT JOIN 
            environment_proxies p ON e.proxy_id = p.id
        LEFT JOIN 
            environment_transfer_history eth ON eth.environment_uuid = e.uuid
        WHERE 
            eth.from_user_uuid = ? AND e.deleted_at IS NULL
        LIMIT ? OFFSET ?
    ";

        let environments: Vec<crate::dto::environment_info::EnvironmentDetailWithResponse> = sqlx::query_as(query)
            .bind(from_user_uuid)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let result_json: Vec<Value> = environments
            .into_iter()
            .map(|env| serde_json::to_value(env).unwrap())
            .collect();

        Ok((total, result_json))
    }

    #[allow(dead_code)]
    pub async fn query_transfer_history_by_environment_uuid(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
    ) -> Result<EnvironmentTransferHistory, Error> {
        let transfer_history: EnvironmentTransferHistory = sqlx::query_as(
            "SELECT * FROM environment_transfer_history WHERE environment_uuid = ? AND deleted_at IS NULL",
        )
        .bind(environment_uuid)
        .fetch_one(pool)
        .await?;

        Ok(transfer_history)
    }

    #[allow(dead_code)]
    pub async fn query_all_transfer_histories(
        pool: &Pool<Sqlite>,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<EnvironmentTransferHistory>), Error> {
        // 获取总数
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environment_transfer_history WHERE deleted_at IS NULL",
        )
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        // 获取分页的转移历史记录列表
        let transfer_histories: Vec<EnvironmentTransferHistory> = sqlx::query_as(
            "SELECT * FROM environment_transfer_history WHERE deleted_at IS NULL LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, transfer_histories))
    }

    #[allow(dead_code)]
    pub async fn update_transfer_history(
        pool: &Pool<Sqlite>,
        transfer_history: &EnvironmentTransferHistory,
    ) -> Result<bool, Error> {
        let sql = "
            UPDATE environment_transfer_history
            SET environment_uuid = ?, from_user_uuid = ?, to_user_uuid = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND deleted_at IS NULL
        ";

        let row = sqlx::query(sql)
            .bind(&transfer_history.environment_uuid)
            .bind(&transfer_history.from_user_uuid)
            .bind(&transfer_history.to_user_uuid)
            .bind(transfer_history.id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete_transfer_history(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        environment_uuid: &str,
    ) -> Result<bool, Error> {
        let sql =
            "UPDATE environment_transfer_history SET deleted_at = CURRENT_TIMESTAMP WHERE environment_uuid = ? and to_user_uuid = ?";

        let row = sqlx::query(sql)
            .bind(environment_uuid)
            .bind(user_uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
