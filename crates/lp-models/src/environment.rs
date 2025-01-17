use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{error::Error, FromRow, Pool, Sqlite};

use crate::{
    dto::environment_info::EnvironmentDetailWithAdvanceCreateRequest,
    environment_fingerprint::EnvironmentFingerprint, environment_proxies::Proxy,
};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentInfo {
    pub id: u32,
    pub uuid: Option<String>,
    pub user_uuid: Option<String>,
    pub team_id: Option<u32>,
    pub proxy: Option<Proxy>,
    pub tag_id: Option<u32>,
    pub group_id: Option<u32>,
    pub fp_info: EnvironmentFingerprint,
    pub name: String,
    pub description: Option<String>,
    pub default_urls: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Environment {
    pub id: u32,                           // 自增ID
    pub uuid: Option<String>,              // UUID
    pub user_uuid: String,                 // 用户UUID
    pub team_id: Option<u32>,              // 团队ID
    pub proxy_id: Option<u32>,             // 代理ID
    pub fp_info_id: Option<u32>,           // 指纹信息ID
    pub group_id: Option<u32>,             // 分组ID
    pub tag_id: Option<u32>,               // tag_id
    pub name: String,                      // 环境名称
    pub description: Option<String>,       // 环境描述
    pub default_urls: Option<String>,      // 默认打开网页
    pub created_at: Option<String>, // 创建时间
    pub updated_at: Option<String>, // 更新时间
    pub lasted_at: Option<String>,  // 最近时间
    pub deleted_at: Option<String>, // 删除时间
}

impl Environment {
    pub async fn simple_insert(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        environment: &EnvironmentInfo,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let fingerprint = &environment.fp_info;
        let sql = "
            INSERT INTO environment_fingerprints (
                user_uuid, browser, ua, os, country, region, city, language, languages, timezone, geography, geo_tips, geo_rule, longitude, latitude, radius, height,
                width, fonts, web_rtc, web_rtc_local_ip, canvas, webgl, hardware_acceleration, webgl_info, audio_context, speech_voices, media, cpu, memory, do_not_track, battery, port_scan
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33
            ) returning id";

        let fp_info_id: u32 = sqlx::query_scalar(sql)
            .bind(user_uuid)
            .bind(&fingerprint.browser)
            .bind(&fingerprint.ua)
            .bind(&fingerprint.os)
            .bind(&fingerprint.country)
            .bind(&fingerprint.region)
            .bind(&fingerprint.city)
            .bind(&fingerprint.language)
            .bind(&fingerprint.languages)
            .bind(&fingerprint.timezone)
            .bind(&fingerprint.geography)
            .bind(&fingerprint.geo_tips)
            .bind(&fingerprint.geo_rule)
            .bind(&fingerprint.longitude)
            .bind(&fingerprint.latitude)
            .bind(&fingerprint.radius)
            .bind(&fingerprint.height)
            .bind(&fingerprint.width)
            .bind(&fingerprint.fonts)
            .bind(&fingerprint.web_rtc)
            .bind(&fingerprint.web_rtc_local_ip)
            .bind(&fingerprint.canvas)
            .bind(&fingerprint.webgl)
            .bind(&fingerprint.hardware_acceleration)
            .bind(&fingerprint.webgl_info)
            .bind(&fingerprint.audio_context)
            .bind(&fingerprint.speech_voices)
            .bind(&fingerprint.media)
            .bind(&fingerprint.cpu)
            .bind(&fingerprint.memory)
            .bind(&fingerprint.do_not_track)
            .bind(&fingerprint.battery)
            .bind(&fingerprint.port_scan)
            .fetch_one(pool)
            .await?;

        let proxy_id = if let Some(proxy) = &environment.proxy {
            let sql = "
            INSERT INTO environment_proxies (
                kind, host, port, username, password, user_uuid
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            ) RETURNING id";

            let proxy_id: u32 = sqlx::query_scalar(sql)
                .bind(&proxy.kind)
                .bind(&proxy.host)
                .bind(&proxy.port)
                .bind(&proxy.username)
                .bind(&proxy.password)
                .bind(user_uuid)
                .fetch_one(&mut *tx)
                .await?;
            Some(proxy_id)
        } else {
            None
        };

        let sql = r#"
        INSERT INTO environments (
            uuid, user_uuid, team_id, proxy_id, fp_info_id, tag_id, group_id, name, description, default_urls
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#;

        let row = sqlx::query(sql)
            .bind(&environment.uuid)
            .bind(user_uuid)
            .bind(environment.team_id)
            .bind(proxy_id)
            .bind(fp_info_id)
            .bind(environment.tag_id)
            .bind(environment.group_id)
            .bind(&environment.name)
            .bind(&environment.description)
            .bind(&environment.default_urls)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() >= 1)
    }

    pub async fn advanced_insert(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        environment: &EnvironmentDetailWithAdvanceCreateRequest,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let sql = "
            INSERT INTO environment_fingerprints (
                user_uuid, browser, ua, os, country, region, city, language, languages, timezone, geography, geo_tips, geo_rule, longitude, latitude, radius, height,
                width, fonts, web_rtc, web_rtc_local_ip, canvas, webgl, hardware_acceleration, webgl_info, audio_context, speech_voices, media, cpu, memory, do_not_track, battery, port_scan
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33
            ) returning id";

        let fp_info_id: u32 = sqlx::query_scalar(sql)
            .bind(user_uuid)
            .bind(&environment.browser)
            .bind(&environment.ua)
            .bind(&environment.os)
            .bind(&environment.country)
            .bind(&environment.region)
            .bind(&environment.city)
            .bind(&environment.language)
            .bind(&environment.languages)
            .bind(&environment.timezone)
            .bind(&environment.geography)
            .bind(&environment.geo_tips)
            .bind(&environment.geo_rule)
            .bind(&environment.longitude)
            .bind(&environment.latitude)
            .bind(&environment.radius)
            .bind(&environment.height)
            .bind(&environment.width)
            .bind(&environment.fonts)
            .bind(&environment.web_rtc)
            .bind(&environment.web_rtc_local_ip)
            .bind(&environment.canvas)
            .bind(&environment.webgl)
            .bind(&environment.hardware_acceleration)
            .bind(&environment.webgl_info)
            .bind(&environment.audio_context)
            .bind(&environment.speech_voices)
            .bind(&environment.media)
            .bind(&environment.cpu)
            .bind(&environment.memory)
            .bind(&environment.do_not_track)
            .bind(&environment.battery)
            .bind(&environment.port_scan)
            .fetch_one(&mut *tx)
            .await?;

        let proxy_id = if let None = &environment.proxy_id {
            let sql = "
            INSERT INTO environment_proxies (
                kind, host, port, username, password, user_uuid
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            ) RETURNING id";

            let proxy_id: u32 = sqlx::query_scalar(sql)
                .bind(&environment.proxy_kind)
                .bind(&environment.proxy_host)
                .bind(&environment.proxy_password)
                .bind(&environment.proxy_username)
                .bind(&environment.proxy_password)
                .bind(user_uuid)
                .fetch_one(&mut *tx)
                .await?;
            Some(proxy_id)
        } else {
            environment.proxy_id.map(|v| v as u32)
        };

        let sql = r#"
        INSERT INTO environments (
            uuid, user_uuid, team_id, proxy_id, fp_info_id, tag_id, group_id, name, description, default_urls
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#;

        let row = sqlx::query(sql)
            .bind(&environment.uuid)
            .bind(user_uuid)
            .bind(environment.team_id)
            .bind(proxy_id)
            .bind(fp_info_id)
            .bind(environment.tag_id)
            .bind(environment.group_id)
            .bind(&environment.name)
            .bind(&environment.description)
            .bind(&environment.default_urls)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() >= 1)
    }

    pub async fn query_environment_details(
        pool: &Pool<Sqlite>,
        uuid: &str,
        user_uuid: &str,
    ) -> Result<Value, sqlx::Error> {
        let query = "
        SELECT 
            e.id, e.uuid, e.user_uuid, e.team_id, e.proxy_id, e.fp_info_id, e.group_id, e.tag_id, e.name, e.description, e.default_urls, e.created_at, e.updated_at, e.lasted_at, e.deleted_at,
            ef.id AS fp_id, ef.browser, ef.ua, ef.os, ef.country, ef.region, ef.city, ef.language, ef.languages, ef.timezone, ef.geography, ef.geo_tips, ef.geo_rule, ef.longitude, ef.latitude, ef.radius, ef.height, ef.width, ef.fonts, ef.web_rtc, ef.web_rtc_local_ip, ef.canvas, ef.webgl, ef.hardware_acceleration, ef.webgl_info, ef.audio_context, ef.speech_voices, ef.media, ef.cpu, ef.memory, ef.do_not_track, ef.battery, ef.port_scan, ef.created_at AS fp_created_at, ef.updated_at AS fp_updated_at, ef.deleted_at AS fp_deleted_at,
            p.kind AS proxy_kind, p.host AS proxy_host, p.port AS proxy_port, p.username AS proxy_username, p.password AS proxy_password, p.user_uuid AS proxy_user_uuid, p.environment_group_id AS proxy_environment_group_id, p.created_at AS proxy_created_at, p.updated_at AS proxy_updated_at, p.deleted_at AS proxy_deleted_at
        FROM 
            environments e
        LEFT JOIN 
            environment_fingerprints ef ON e.fp_info_id = ef.id
        LEFT JOIN 
            environment_proxies p ON e.proxy_id = p.id
        WHERE 
            e.uuid = $1 AND e.user_uuid = $2 AND e.deleted_at IS NULL;
    ";

        let environment_with_details: crate::dto::environment_info::EnvironmentDetailWithResponse =
            sqlx::query_as(query)
                .bind(uuid)
                .bind(user_uuid)
                .fetch_one(pool)
                .await?;

        let result_json = lp_commons::util::struct_to_json_value(environment_with_details);

        Ok(result_json)
    }

    #[allow(dead_code)]
    pub async fn query_by_uuid(pool: &Pool<Sqlite>, uuid: &str) -> Result<Environment, Error> {
        let environment: Environment =
            sqlx::query_as("SELECT * FROM environments WHERE uuid = $1 AND deleted_at IS NULL")
                .bind(uuid)
                .fetch_one(pool)
                .await?;

        Ok(environment)
    }

    #[allow(dead_code)]
    pub async fn query_environments_by_group_id(
        pool: &Pool<Sqlite>,
        group_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Value>), Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments WHERE group_id = $1 AND deleted_at IS NULL",
        )
        .bind(group_id)
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
            e.id, e.uuid, e.user_uuid, e.team_id, e.proxy_id, e.fp_info_id, e.tag_id, e.group_id, eg.name AS group_name, e.name, e.description, e.default_urls, e.created_at, e.updated_at, e.lasted_at, e.deleted_at,
            ef.id AS fp_id, ef.browser, ef.ua, ef.os, ef.country, ef.region, ef.city, ef.language, ef.languages, ef.timezone, ef.geography, ef.geo_tips, ef.geo_rule, ef.longitude, ef.latitude, ef.radius, ef.height, ef.width, ef.fonts, ef.web_rtc, ef.web_rtc_local_ip, ef.canvas, ef.webgl, ef.hardware_acceleration, ef.webgl_info, ef.audio_context, ef.speech_voices, ef.media, ef.cpu, ef.memory, ef.do_not_track, ef.battery, ef.port_scan, ef.created_at AS fp_created_at, ef.updated_at AS fp_updated_at, ef.deleted_at AS fp_deleted_at,
            p.kind AS proxy_kind, p.host AS proxy_host, p.port AS proxy_port, p.username AS proxy_username, p.password AS proxy_password, p.user_uuid AS proxy_user_uuid, p.environment_group_id AS proxy_environment_group_id, p.created_at AS proxy_created_at, p.updated_at AS proxy_updated_at, p.deleted_at AS proxy_deleted_at
        FROM 
            environments e
        LEFT JOIN 
            environment_fingerprints ef ON e.fp_info_id = ef.id
        LEFT JOIN 
            environment_groups eg ON e.group_id = eg.id
        LEFT JOIN 
            environment_proxies p ON e.proxy_id = p.id
        WHERE 
            e.group_id = $1 AND e.deleted_at IS NULL
        LIMIT $2 OFFSET $3;
    ";

        let environments_with_details: Vec<
            crate::dto::environment_info::EnvironmentDetailWithResponse,
        > = sqlx::query_as(query)
            .bind(group_id)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?;

        let result_json: Vec<Value> = environments_with_details
            .into_iter()
            .map(|env| serde_json::to_value(env).unwrap())
            .collect();

        Ok((total, result_json))
    }

    pub async fn query_environments_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Value>), sqlx::Error> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments WHERE user_uuid = $1 AND deleted_at IS NULL",
        )
        .bind(user_uuid)
        .fetch_one(pool)
        .await?;

        let (offset, page_size) =
            lp_commons::util::calculate_pagination(page_num, page_size, total);

        let query = "
        SELECT 
            e.id, e.uuid, e.user_uuid, e.team_id, e.proxy_id, e.fp_info_id, e.tag_id, e.group_id, eg.name AS group_name, e.name, e.description, e.default_urls, e.created_at, e.updated_at, e.lasted_at, e.deleted_at,
            ef.id AS fp_id, ef.browser, ef.ua, ef.os, ef.country, ef.region, ef.city, ef.language, ef.languages, ef.timezone, ef.geography, ef.geo_tips, ef.geo_rule, ef.longitude, ef.latitude, ef.radius, ef.height, ef.width, ef.fonts, ef.web_rtc, ef.web_rtc_local_ip, ef.canvas, ef.webgl, ef.hardware_acceleration, ef.webgl_info, ef.audio_context, ef.speech_voices, ef.media, ef.cpu, ef.memory, ef.do_not_track, ef.battery, ef.port_scan, ef.created_at AS fp_created_at, ef.updated_at AS fp_updated_at, ef.deleted_at AS fp_deleted_at,
            p.kind AS proxy_kind, p.host AS proxy_host, p.port AS proxy_port, p.username AS proxy_username, p.password AS proxy_password, p.user_uuid AS proxy_user_uuid, p.environment_group_id AS proxy_environment_group_id, p.created_at AS proxy_created_at, p.updated_at AS proxy_updated_at, p.deleted_at AS proxy_deleted_at
        FROM 
            environments e
        LEFT JOIN 
            environment_fingerprints ef ON e.fp_info_id = ef.id
        LEFT JOIN 
            environment_groups eg ON e.group_id = eg.id
        LEFT JOIN 
            environment_proxies p ON e.proxy_id = p.id
        WHERE 
            e.user_uuid = $1 AND e.deleted_at IS NULL
        LIMIT $2 OFFSET $3;
        ";

        let environments_with_details: Vec<
            crate::dto::environment_info::EnvironmentDetailWithResponse,
        > = sqlx::query_as(query)
            .bind(user_uuid)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?;

        let result_json: Vec<Value> = environments_with_details
            .into_iter()
            .map(|env| serde_json::to_value(env).unwrap())
            .collect();

        Ok((total, result_json))
    }

    #[allow(dead_code)]
    pub async fn modify_proxy(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        environment_uuid: &str,
        proxy: &Proxy,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;
        let sql = "
            INSERT INTO environment_proxies (
                kind, host, port, username, password, user_uuid
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            ) RETURNING id";

        let proxy_id: u32 = sqlx::query_scalar(sql)
            .bind(&proxy.kind)
            .bind(&proxy.host)
            .bind(&proxy.port)
            .bind(&proxy.username)
            .bind(&proxy.password)
            .bind(user_uuid)
            .fetch_one(&mut *tx)
            .await?;

        let sql = r#"
        UPDATE environments SET
            proxy_id = $1
        WHERE uuid = $2
        "#;

        let row = sqlx::query(sql)
            .bind(proxy_id)
            .bind(environment_uuid)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn modify_basic_info(
        pool: &Pool<Sqlite>,
        uuid: &str,
        name: &str,
        description: Option<String>,
    ) -> Result<bool, Error> {
        let sql = "
            UPDATE environments
            SET name = $1, description = $2, updated_at = CURRENT_TIMESTAMP
            WHERE uuid = $3 AND deleted_at IS NULL
        ";

        let row = sqlx::query(sql)
            .bind(name)
            .bind(description)
            .bind(uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn modify_and_other_info(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        environment_uuid: &str,
        environment: &EnvironmentInfo,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;


         let fingerprint = &environment.fp_info;
        let sql = "
        UPDATE environment_fingerprints
        SET browser = $1, ua = $2, os = $3, country = $4, region = $5, city = $6, language = $7, languages = $8, timezone = $9, geography = $10, geo_tips = $11, geo_rule = $12, longitude = $13, latitude = $14, radius = $15, height = $16, width = $17, fonts = $18, web_rtc = $19, web_rtc_local_ip = $20, canvas = $21, webgl = $22, hardware_acceleration = $23, webgl_info = $24, audio_context = $25, speech_voices = $26, media = $27, cpu = $28, memory = $29, do_not_track = $30, battery = $31, port_scan = $32, updated_at = CURRENT_TIMESTAMP
        WHERE user_uuid = $33 AND id = $34;
        ";

        sqlx::query(sql)
            .bind(&fingerprint.browser)
            .bind(&fingerprint.ua)
            .bind(&fingerprint.os)
            .bind(&fingerprint.country)
            .bind(&fingerprint.region)
            .bind(&fingerprint.city)
            .bind(&fingerprint.language)
            .bind(&fingerprint.languages)
            .bind(&fingerprint.timezone)
            .bind(&fingerprint.geography)
            .bind(&fingerprint.geo_tips)
            .bind(&fingerprint.geo_rule)
            .bind(&fingerprint.longitude)
            .bind(&fingerprint.latitude)
            .bind(&fingerprint.radius)
            .bind(&fingerprint.height)
            .bind(&fingerprint.width)
            .bind(&fingerprint.fonts)
            .bind(&fingerprint.web_rtc)
            .bind(&fingerprint.web_rtc_local_ip)
            .bind(&fingerprint.canvas)
            .bind(&fingerprint.webgl)
            .bind(&fingerprint.hardware_acceleration)
            .bind(&fingerprint.webgl_info)
            .bind(&fingerprint.audio_context)
            .bind(&fingerprint.speech_voices)
            .bind(&fingerprint.media)
            .bind(&fingerprint.cpu)
            .bind(&fingerprint.memory)
            .bind(&fingerprint.do_not_track)
            .bind(&fingerprint.battery)
            .bind(&fingerprint.port_scan)
            .bind(user_uuid)
            .bind(fingerprint.id)
            .execute(&mut *tx)
            .await?;

        let proxy_sql = if let Some(proxy) = &environment.proxy {
            let sql = "
            UPDATE environment_proxies SET
                kind = $1, host = $2, port = $3, username = $4, password = $5, user_uuid = $6
            WHERE id = $7 RETURNING id
            ";

            let proxy_id: u32 = sqlx::query_scalar(sql)
                .bind(&proxy.kind)
                .bind(&proxy.host)
                .bind(&proxy.port)
                .bind(&proxy.username)
                .bind(&proxy.password)
                .bind(&proxy.user_uuid)
                .bind(proxy.id)
                .fetch_one(&mut *tx)
                .await?;

            format!("proxy_id = {},", proxy_id)
        } else {
            format!("")
        };

        let rows_affected = sqlx::query(&format!(
            "
        UPDATE environments SET
            {} fp_info_id = $1, name = $2, description = $3, default_urls = $4
        WHERE uuid = $5
        ",
            proxy_sql
        ))
        .bind(environment.fp_info.id)
        .bind(&environment.name)
        .bind(&environment.description)
        .bind(&environment.default_urls)
        .bind(environment_uuid)
        .execute(&mut *tx)
        .await?
        .rows_affected();

        tx.commit().await?;

        Ok(rows_affected >= 1)
    }

    #[allow(dead_code)]
    pub async fn query_environments_by_team_id_and_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        team_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Value>), Error> {
        let relation_exists: (i64,) = sqlx::query_as(
            "SELECT count(1) FROM user_team_relation
         WHERE user_uuid = $1 AND team_id = $2 AND blocked != 1 AND deleted_at IS NULL",
        )
        .bind(user_uuid)
        .bind(team_id)
        .fetch_one(pool)
        .await?;

        if relation_exists.0 == 0 {
            return Err(Error::RowNotFound);
        }

        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments WHERE team_id = $1 AND deleted_at IS NULL",
        )
        .bind(team_id)
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
            e.id, e.uuid, e.user_uuid, e.team_id, e.proxy_id, e.fp_info_id, e.tag_id, e.group_id, e.name, e.description, e.default_urls, e.created_at, e.updated_at, e.lasted_at, e.deleted_at,
            ef.id AS fp_id, ef.browser, ef.ua, ef.os, ef.country, ef.region, ef.city, ef.language, ef.languages, ef.timezone, ef.geography, ef.geo_tips, ef.geo_rule, ef.longitude, ef.latitude, ef.radius, ef.height, ef.width, ef.fonts, ef.web_rtc, ef.web_rtc_local_ip, ef.canvas, ef.webgl, ef.hardware_acceleration, ef.webgl_info, ef.audio_context, ef.speech_voices, ef.media, ef.cpu, ef.memory, ef.do_not_track, ef.battery, ef.port_scan, ef.created_at AS fp_created_at, ef.updated_at AS fp_updated_at, ef.deleted_at AS fp_deleted_at,
            p.kind AS proxy_kind, p.host AS proxy_host, p.port AS proxy_port, p.username AS proxy_username, p.password AS proxy_password, p.user_uuid AS proxy_user_uuid, p.environment_group_id AS proxy_environment_group_id, p.created_at AS proxy_created_at, p.updated_at AS proxy_updated_at, p.deleted_at AS proxy_deleted_at
        FROM 
            environments e
        LEFT JOIN 
            environment_fingerprints ef ON e.fp_info_id = ef.id
        LEFT JOIN 
            environment_proxies p ON e.proxy_id = p.id
        WHERE 
            e.team_id = $1 AND e.deleted_at IS NULL
        LIMIT $2 OFFSET $3;
        ";

        let environments_with_details: Vec<
            crate::dto::environment_info::EnvironmentDetailWithResponse,
        > = sqlx::query_as(query)
            .bind(team_id)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?;

        let result_json: Vec<Value> = environments_with_details
            .into_iter()
            .map(|env| serde_json::to_value(env).unwrap())
            .collect();

        Ok((total, result_json))
    }

    #[allow(dead_code)]
    pub async fn move_environment_to_group(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
        group_id: u32,
    ) -> Result<bool, Error> {
        let row = sqlx::query(
            r#"
            UPDATE environments
            SET
                group_id = $1,
                updated_at = CURRENT_TIMESTAMP
            WHERE uuid = $2;
            "#,
        )
        .bind(group_id)
        .bind(environment_uuid)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn delete_and_move_to_trash(
        pool: &Pool<Sqlite>,
        environment_uuid: &str,
        from_user_uuid: &str,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        sqlx::query(
            "INSERT INTO environment_trash (environment_uuid, from_user_uuid) VALUES ($1, $2)",
        )
        .bind(environment_uuid)
        .bind(from_user_uuid)
        .execute(&mut *tx)
        .await?;

        let row = sqlx::query(
            "UPDATE environments SET deleted_at = CURRENT_TIMESTAMP WHERE uuid = $1 AND user_uuid = $2",
        )
        .bind(environment_uuid)
        .bind(from_user_uuid)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }
}
