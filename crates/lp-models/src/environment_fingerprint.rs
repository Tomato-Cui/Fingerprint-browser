use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct EnvironmentFingerprint {
    pub id: Option<i32>,                  // 自增ID
    pub user_uuid: Option<String>,        // 用户UUID
    pub browser: String,                  // 浏览器信息
    pub ua: String,                       // 用户代理（User-Agent）
    pub os: String,                       // 操作系统信息
    pub country: Option<String>,          // 国家
    pub region: Option<String>,           // 地区
    pub city: Option<String>,             // 城市
    pub language: Option<String>,         // 语言类型
    pub languages: Option<String>,        // 语言列表
    pub timezone: Option<String>,         // 时区
    pub geography: Option<String>,        // 地理位置信息
    pub geo_tips: Option<String>,         // 地理位置提示
    pub geo_rule: Option<String>,         // 地理位置规则
    pub longitude: Option<f64>,           // 经度（数值类型，保留6位小数）
    pub latitude: Option<f64>,            // 纬度（数值类型，保留6位小数）
    pub radius: Option<f64>,              // 半径（数值类型）
    pub height: Option<i32>,              // 屏幕高度（整数类型）
    pub width: Option<i32>,               // 屏幕宽度（整数类型）
    pub fonts: Option<String>,            // 字体列表
    pub web_rtc: bool,                    // WebRTC 配置（布尔类型）
    pub web_rtc_local_ip: Option<String>, // WebRTC 本地IP（IP地址类型）
    pub canvas: Option<String>,           // Canvas 指纹
    pub webgl: bool,                      // WebGL 配置（布尔类型）
    pub hardware_acceleration: bool,      // 硬件加速（布尔类型）
    pub webgl_info: Option<String>,       // WebGL 信息（JSON格式文本）
    pub audio_context: bool,              // 音频上下文（布尔类型）
    pub speech_voices: bool,              // 语音支持（布尔类型）
    pub media: bool,                      // 媒体支持（布尔类型）
    pub cpu: i32,                         // CPU 核心数（整数类型）
    pub memory: i32,                      // 内存大小（整数类型，单位：GB）
    pub do_not_track: bool,               // 是否启用 Do Not Track（布尔类型）
    pub battery: bool,                    // 电池状态（布尔类型）
    pub port_scan: bool,                  // 端口扫描（布尔类型）
    pub created_at: Option<String>,       // 创建时间
    pub updated_at: Option<String>,       // 更新时间
    pub deleted_at: Option<String>,       // 删除时间
}

impl EnvironmentFingerprint {
    #[allow(dead_code)]
    pub async fn insert(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        fingerprint: &EnvironmentFingerprint,
    ) -> Result<u32, Error> {
        let sql = "
            INSERT INTO environment_fingerprints (
                user_uuid, browser, ua, os, country, region, city, language, languages, timezone, geography, geo_tips, geo_rule, longitude, latitude, radius, height,
                width, fonts, web_rtc, web_rtc_local_ip, canvas, webgl, hardware_acceleration, webgl_info, audio_context, speech_voices, media, cpu, memory, do_not_track, battery, port_scan
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33
            ) returning id";

        let id: u32 = sqlx::query_scalar(sql)
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

        Ok(id)
    }

    #[allow(dead_code)]
    pub async fn query_by_id(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        id: u32,
    ) -> Result<EnvironmentFingerprint, Error> {
        let fingerprint: EnvironmentFingerprint =
            sqlx::query_as("SELECT * FROM environment_fingerprints WHERE id = $1 AND user_uuid = $2 AND deleted_at IS NULL")
                .bind(id)
                .bind(user_uuid)
                .fetch_one(pool)
                .await?;

        Ok(fingerprint)
    }

    #[allow(dead_code)]
    pub async fn default_fingerprint(pool: &Pool<Sqlite>) -> Result<EnvironmentFingerprint, Error> {
        let fingerprint: EnvironmentFingerprint = sqlx::query_as(
            "SELECT * FROM environment_fingerprints WHERE id = 1 AND deleted_at IS NULL",
        )
        .fetch_one(pool)
        .await?;

        Ok(fingerprint)
    }

    #[allow(dead_code)]
    pub async fn query_by_user_uuid(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<EnvironmentFingerprint>), Error> {
        let (total,): (i64,) = sqlx::query_as("SELECT count(1) FROM environment_fingerprints WHERE user_uuid = ? AND deleted_at IS NULL")
            .bind(user_uuid)
            .fetch_one(pool)
            .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let fingerprints: Vec<EnvironmentFingerprint> = sqlx::query_as("SELECT * FROM environment_fingerprints WHERE user_uuid = ? AND deleted_at IS NULL LIMIT ? OFFSET ?")
            .bind(user_uuid)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok((total, fingerprints))
    }

    #[allow(dead_code)]
    pub async fn update(
        pool: &Pool<Sqlite>,
        id: u32,
        user_uuid: &str,
        fingerprint: &EnvironmentFingerprint,
    ) -> Result<bool, Error> {
        let sql = "
        UPDATE environment_fingerprints
            SET browser = $1, ua = $2, os = $3, country = $4, region = $5, city = $6, language = $7, languages = $8, timezone = $9, geography = $10, geo_tips = $11, geo_rule = $12,
                longitude = $13, latitude = $14, radius = $15, height = $16, width = $17, fonts = $18, web_rtc = $19, web_rtc_local_ip = $20, canvas = $21, webgl = $22, hardware_acceleration = $23, webgl_info = $24, audio_context = $25, 
                speech_voices = $26, media = $27, cpu = $28, memory = $29, do_not_track = $30, battery = $31, port_scan = $32, 
                updated_at = CURRENT_TIMESTAMP
        WHERE id = $33 AND user_uuid = $34 AND deleted_at IS NULL;
        ";

        let row = sqlx::query(sql)
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
            .bind(id)
            .bind(user_uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn update_by_col(
        pool: &Pool<Sqlite>,
        user_uuid: &str,
        id: u32,
        col_name: &str,
        col_value: &str,
    ) -> Result<bool, Error> {
        if col_name.is_empty() {
            return Err(sqlx::error::Error::ColumnNotFound(format!(
                "{} column not found",
                col_name
            )));
        }
        let row = sqlx::query(&format!(
            "UPDATE environment_fingerprints SET {} = ? WHERE id = ? AND user_uuid = ? AND deleted_at IS NULL",
            col_name
        ))
        .bind(col_value)
        .bind(id)
        .bind(user_uuid)
        .execute(pool)
        .await?;
        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, user_uuid: &str, id: u32) -> Result<bool, Error> {
        let sql = "UPDATE environment_fingerprints SET deleted_at = CURRENT_TIMESTAMP WHERE id = ? AND user_uuid = ?";

        let row = sqlx::query(sql)
            .bind(id)
            .bind(user_uuid)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
