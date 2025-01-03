use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{error::Error, FromRow, Pool, Sqlite};

use crate::{environment_fingerprint::EnvironmentFingerprint, environment_proxies::Proxy};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentInfo {
    pub id: i32,
    pub uuid: Option<String>,
    pub user_uuid: Option<String>,
    pub team_id: Option<i32>,
    pub proxy: Option<Proxy>,
    pub fp_info: EnvironmentFingerprint,
    pub name: String,
    pub description: Option<String>,
    pub default_urls: Option<String>,
    pub proxy_enable: i8,
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Environment {
    pub id: i32,                      // 自增ID
    pub uuid: Option<String>,         // UUID
    pub user_uuid: String,            // 用户UUID
    pub team_id: Option<i32>,         // 团队ID
    pub proxy_id: Option<i32>,        // 代理ID
    pub fp_info_id: Option<i32>,      // 指纹信息ID
    pub group_id: Option<i32>,        // 分组ID
    pub name: String,                 // 环境名称
    pub description: Option<String>,  // 环境描述
    pub default_urls: Option<String>, // 默认打开网页
    pub proxy_enable: i8,             // 代理启用
    pub created_at: Option<String>,   // 创建时间
    pub updated_at: Option<String>,   // 更新时间
    pub lasted_at: Option<String>,    // 最近时间
    pub deleted_at: Option<String>,   // 删除时间
}

impl Environment {
    #[allow(dead_code)]
    pub async fn insert_and_other_info(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        environment: &EnvironmentInfo,
    ) -> Result<bool, Error> {
        let mut tx = pool.begin().await?;

        let fingerprint = &environment.fp_info;
        let sql = "
            INSERT INTO environment_fingerprints (
                user_uuid, browser, ua, os, country, region, city, language_type, languages, gmt, geography, geo_tips, geo_rule, longitude, latitude, radius, height,
                width, fonts_type, fonts, font_fingerprint, web_rtc, web_rtc_local_ip, canvas, webgl, hardware_acceleration,
                webgl_info, audio_context, speech_voices, media, cpu, memory, do_not_track, battery, port_scan, white_list
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            ) RETURNING id";

        let fp_info_id: u32 = sqlx::query_scalar(sql)
            .bind(user_uuid)
            .bind(&fingerprint.browser)
            .bind(&fingerprint.ua)
            .bind(&fingerprint.os)
            .bind(&fingerprint.country)
            .bind(&fingerprint.region)
            .bind(&fingerprint.city)
            .bind(&fingerprint.language_type)
            .bind(&fingerprint.languages)
            .bind(&fingerprint.gmt)
            .bind(&fingerprint.geography)
            .bind(&fingerprint.geo_tips)
            .bind(&fingerprint.geo_rule)
            .bind(&fingerprint.longitude)
            .bind(&fingerprint.latitude)
            .bind(&fingerprint.radius)
            .bind(&fingerprint.height)
            .bind(&fingerprint.width)
            .bind(&fingerprint.fonts_type)
            .bind(&fingerprint.fonts)
            .bind(&fingerprint.font_fingerprint)
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
            .bind(&fingerprint.white_list)
            .fetch_one(&mut *tx)
            .await?;

        let proxy_id = if let Some(proxy) = &environment.proxy {
            let sql = "
            INSERT INTO environment_proxies (
                kind, host, port, username, password, user_uuid
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?
            ) RETURNING id";

            let proxy_id = sqlx::query_scalar(sql)
                .bind(&proxy.kind)
                .bind(&proxy.host)
                .bind(&proxy.port)
                .bind(&proxy.username)
                .bind(&proxy.password)
                .bind(&proxy.user_uuid)
                .fetch_one(pool)
                .await?;
            proxy_id
        } else {
            0
        };

        let sql = r#"
        INSERT INTO environments (
            uuid, user_uuid, team_id, proxy_id, fp_info_id, name, description, default_urls, 
            proxy_enable
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#;

        let row = sqlx::query(sql)
            .bind(&environment.uuid)
            .bind(&environment.user_uuid)
            .bind(environment.team_id)
            .bind(proxy_id)
            .bind(fp_info_id)
            .bind(&environment.name)
            .bind(&environment.description)
            .bind(&environment.default_urls)
            .bind(environment.proxy_enable)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn insert(pool: &Pool<Sqlite>, environment: &Environment) -> Result<bool, Error> {
        let fp_info_id = EnvironmentFingerprint::insert(
            pool,
            &environment.user_uuid,
            &EnvironmentFingerprint {
                ..Default::default()
            },
        )
        .await?;
        let proxy_id = Proxy::insert_proxy(
            pool,
            &&Proxy {
                user_uuid: Some(environment.user_uuid.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let sql = r#"
        INSERT INTO environments (
            uuid, user_uuid, team_id, proxy_id, fp_info_id, group_id, name, description, default_urls, 
            proxy_enable
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#;

        let row = sqlx::query(sql)
            .bind(&environment.uuid)
            .bind(&environment.user_uuid)
            .bind(environment.team_id)
            .bind(proxy_id)
            .bind(fp_info_id)
            .bind(environment.group_id)
            .bind(&environment.name)
            .bind(&environment.description)
            .bind(&environment.default_urls)
            .bind(environment.proxy_enable)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() >= 1)
    }

    pub async fn query_environment_details(
        pool: &Pool<Sqlite>,
        uuid: &str,
        user_uuid: &str,
    ) -> Result<Value, sqlx::Error> {
        let query = "
        SELECT 
            e.id, e.uuid, e.user_uuid, e.team_id, e.proxy_id, e.fp_info_id, e.group_id, e.name, e.description, e.default_urls, e.proxy_enable, e.created_at, e.updated_at, e.lasted_at, e.deleted_at,
            ef.id AS fp_id, ef.browser, ef.ua, ef.os, ef.country, ef.region, ef.city, ef.language_type, ef.languages, ef.gmt, ef.geography, ef.geo_tips, ef.geo_rule, ef.longitude, ef.latitude, ef.radius, ef.height, ef.width, ef.fonts_type, ef.fonts, ef.font_fingerprint, ef.web_rtc, ef.web_rtc_local_ip, ef.canvas, ef.webgl, ef.hardware_acceleration, ef.webgl_info, ef.audio_context, ef.speech_voices, ef.media, ef.cpu, ef.memory, ef.do_not_track, ef.battery, ef.port_scan, ef.white_list, ef.created_at AS fp_created_at, ef.updated_at AS fp_updated_at, ef.deleted_at AS fp_deleted_at,
            p.kind AS proxy_kind, p.host AS proxy_host, p.port AS proxy_port, p.username AS proxy_username, p.password AS proxy_password, p.user_uuid AS proxy_user_uuid, p.environment_group_id AS proxy_environment_group_id, p.created_at AS proxy_created_at, p.updated_at AS proxy_updated_at, p.deleted_at AS proxy_deleted_at
        FROM 
            environments e
        LEFT JOIN 
            environment_fingerprints ef ON e.fp_info_id = ef.id
        LEFT JOIN 
            environment_proxies p ON e.proxy_id = p.id
        WHERE 
            e.uuid = ? AND e.user_uuid = ? AND e.deleted_at IS NULL
    ";

        let environment_with_details: crate::dto::environment_info::EnvironmentWithDetails =
            sqlx::query_as(query)
                .bind(uuid)
                .bind(user_uuid)
                .fetch_one(pool)
                .await?;

        let result_json = commons::util::struct_to_json_value(environment_with_details);

        Ok(result_json)
    }

    #[allow(dead_code)]
    pub async fn query_by_uuid(pool: &Pool<Sqlite>, uuid: &str) -> Result<Environment, Error> {
        let environment: Environment =
            sqlx::query_as("SELECT * FROM environments WHERE uuid = ? AND deleted_at IS NULL")
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
            "SELECT count(1) FROM environments WHERE group_id = ? AND deleted_at IS NULL",
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
        WHERE 
            e.group_id = ? AND e.deleted_at IS NULL
        LIMIT ? OFFSET ?
    ";

        let environments_with_details: Vec<crate::dto::environment_info::EnvironmentWithInfo> =
            sqlx::query_as(query)
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
            "SELECT count(1) FROM environments WHERE user_uuid = ? AND deleted_at IS NULL",
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
        WHERE 
            e.user_uuid = ? AND e.deleted_at IS NULL
        LIMIT ? OFFSET ?
    ";

        let environments_with_details: Vec<crate::dto::environment_info::EnvironmentWithInfo> =
            sqlx::query_as(query)
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
                ?, ?, ?, ?, ?, ?
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
            proxy_id = ?
        WHERE uuid = ?
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
            SET name = ?, description = ?, updated_at = CURRENT_TIMESTAMP
            WHERE uuid = ? AND deleted_at IS NULL
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
            SET browser = ?, ua = ?, os = ?, country = ?, region = ?, city = ?, language_type = ?, languages = ?, gmt = ?, geography = ?, geo_tips = ?, geo_rule = ?,
                longitude = ?, latitude = ?, radius = ?, height = ?, width = ?, fonts_type = ?, fonts = ?, font_fingerprint = ?, 
                web_rtc = ?, web_rtc_local_ip = ?, canvas = ?, webgl = ?, hardware_acceleration = ?, webgl_info = ?, audio_context = ?, 
                speech_voices = ?, media = ?, cpu = ?, memory = ?, do_not_track = ?, battery = ?, port_scan = ?, white_list = ?, 
                updated_at = CURRENT_TIMESTAMP
        WHERE user_uuid = ? AND id = ?;
        ";

        sqlx::query(sql)
            .bind(&fingerprint.browser)
            .bind(&fingerprint.ua)
            .bind(&fingerprint.os)
            .bind(&fingerprint.country)
            .bind(&fingerprint.region)
            .bind(&fingerprint.city)
            .bind(&fingerprint.language_type)
            .bind(&fingerprint.languages)
            .bind(&fingerprint.gmt)
            .bind(&fingerprint.geography)
            .bind(&fingerprint.geo_tips)
            .bind(&fingerprint.geo_rule)
            .bind(&fingerprint.longitude)
            .bind(&fingerprint.latitude)
            .bind(&fingerprint.radius)
            .bind(&fingerprint.height)
            .bind(&fingerprint.width)
            .bind(&fingerprint.fonts_type)
            .bind(&fingerprint.fonts)
            .bind(&fingerprint.font_fingerprint)
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
            .bind(&fingerprint.white_list)
            .bind(user_uuid)
            .bind(fingerprint.id)
            .execute(&mut *tx)
            .await?;

        let proxy_sql = if let Some(proxy) = &environment.proxy {
            let sql = "
            UPDATE environment_proxies SET
                kind = ?, host = ?, port = ?, username = ?, password = ?, user_uuid = ?
            WHERE id = ? RETURNING id
            ";

            let proxy_id: u32 = sqlx::query_scalar(sql)
                .bind(&proxy.kind)
                .bind(&proxy.host)
                .bind(&proxy.port)
                .bind(&proxy.username)
                .bind(&proxy.password)
                .bind(&proxy.user_uuid)
                .bind(proxy.id)
                .fetch_one(pool)
                .await?;

            format!("proxy_id = {},", proxy_id)
        } else {
            format!("")
        };

        let rows_affected = sqlx::query(&format!(
            "
        UPDATE environments SET
            {} fp_info_id = ?, name = ?, description = ?, default_urls = ?, proxy_enable = ?
        WHERE uuid = ?
        ",
            proxy_sql
        ))
        .bind(environment.fp_info.id)
        .bind(&environment.name)
        .bind(&environment.description)
        .bind(&environment.default_urls)
        .bind(environment.proxy_enable)
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
         WHERE user_uuid = ? AND team_id = ? AND blocked != 1 AND deleted_at IS NULL",
        )
        .bind(user_uuid)
        .bind(team_id)
        .fetch_one(pool)
        .await?;

        if relation_exists.0 == 0 {
            return Err(Error::RowNotFound);
        }

        let (total,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM environments WHERE team_id = ? AND deleted_at IS NULL",
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
            e.id, e.uuid, e.user_uuid, e.team_id, e.proxy_id, e.fp_info_id, e.group_id, e.name, e.description, e.default_urls, e.proxy_enable, e.created_at, e.updated_at, e.lasted_at, e.deleted_at,
            ef.id AS fp_id, ef.browser, ef.ua, ef.os, ef.country, ef.region, ef.city, ef.language_type, ef.languages, ef.gmt, ef.geography, ef.geo_tips, ef.geo_rule, ef.longitude, ef.latitude, ef.radius, ef.height, ef.width, ef.fonts_type, ef.fonts, ef.font_fingerprint, ef.web_rtc, ef.web_rtc_local_ip, ef.canvas, ef.webgl, ef.hardware_acceleration, ef.webgl_info, ef.audio_context, ef.speech_voices, ef.media, ef.cpu, ef.memory, ef.do_not_track, ef.battery, ef.port_scan, ef.white_list, ef.created_at AS fp_created_at, ef.updated_at AS fp_updated_at, ef.deleted_at AS fp_deleted_at,
            p.kind AS proxy_kind, p.host AS proxy_host, p.port AS proxy_port, p.username AS proxy_username, p.password AS proxy_password, p.user_uuid AS proxy_user_uuid, p.environment_group_id AS proxy_environment_group_id, p.created_at AS proxy_created_at, p.updated_at AS proxy_updated_at, p.deleted_at AS proxy_deleted_at
        FROM 
            environments e
        LEFT JOIN 
            environment_fingerprints ef ON e.fp_info_id = ef.id
        LEFT JOIN 
            environment_proxies p ON e.proxy_id = p.id
        WHERE 
            e.team_id = ? AND e.deleted_at IS NULL
        LIMIT ? OFFSET ?
    ";

        let environments_with_details: Vec<crate::dto::environment_info::EnvironmentWithInfo> =
            sqlx::query_as(query)
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
                group_id = ?,
                updated_at = DATETIME('now')
            WHERE uuid = ?;
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
            "INSERT INTO environment_trash (environment_uuid, from_user_uuid) VALUES (?, ?)",
        )
        .bind(environment_uuid)
        .bind(from_user_uuid)
        .execute(&mut *tx)
        .await?;

        let row = sqlx::query(
            "UPDATE environments SET deleted_at = DATETIME('now') WHERE uuid = ? and user_uuid = ?",
        )
        .bind(environment_uuid)
        .bind(from_user_uuid)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row.rows_affected() == 1)
    }
}
