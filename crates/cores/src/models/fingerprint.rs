use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{apis::PageParam, database::get_db, errors::ApplicationServerError};

#[derive(Debug, Deserialize, Serialize, FromRow, Default)]
pub struct Fingerprint {
    pub id: Option<i32>,                  // 自增ID
    pub ua_version: i32,                  // UA版本范围102~124
    pub ua: String,                       // 自定义UA
    pub language_type: i32,               // 语言类型 0-跟随IP，1-自定义，2-跟随电脑
    pub languages: String,                // 渲染语言
    pub gmt: String,                      // 时区
    pub geography: String,                // 地理
    pub geo_tips: i32,                    // 地理位置请求行为
    pub geo_rule: i32,                    // 地理位置规则
    pub longitude: Option<String>,        // 自定义经度
    pub latitude: Option<String>,         // 自定义纬度
    pub radius: Option<i32>,              // 自定义半径
    pub height: Option<i32>,              // 分辨率高
    pub width: Option<i32>,               // 分辨率宽
    pub fonts_type: i32,                  // 字体列表保护 0-隐私，1-真实
    pub fonts: Option<String>,            // 字体列表
    pub font_fingerprint: i32,            // 字体指纹
    pub web_rtc: i32,                     // WebRTC配置
    pub web_rtc_local_ip: Option<String>, // 内网IP
    pub canvas: i32,                      // Canvas隐私保护
    pub webgl: i32,                       // WebGL隐私保护
    pub hardware_acceleration: i32,       // 硬件加速
    pub webgl_info: i32,                  // WebGL信息
    pub audio_context: i32,               // AudioContext隐私保护
    pub speech_voices: i32,               // SpeechVoices
    pub media: i32,                       // 媒体设备隐私保护
    pub cpu: i32,                         // CPU
    pub memory: i32,                      // 内存
    pub do_not_track: i32,                // 追踪设置
    pub battery: i32,                     // 电池隐私保护
    pub port_scan: i32,                   // 本地端口扫码
    pub white_list: Option<String>,       // 端口白名单
    pub created_at: Option<String>,       // 创建时间
    pub updated_at: Option<String>,       // 更新时间
    pub deleted_at: Option<String>,       // 删除时间
}

impl Fingerprint {
    pub async fn insert(fingerprint: Fingerprint) -> Result<bool, ApplicationServerError> {
        let sql = "
            INSERT INTO fingerprints (
                ua_version,
                ua,
                language_type,
                languages,
                gmt,
                geography,
                geo_tips,
                geo_rule,
                longitude,
                latitude,
                radius,
                height,
                width,
                fonts_type,
                fonts,
                font_fingerprint,
                web_rtc,
                web_rtc_local_ip,
                canvas,
                webgl,
                hardware_acceleration,
                webgl_info,
                audio_context,
                speech_voices,
                media,
                cpu,
                memory,
                do_not_track,
                battery,
                port_scan,
                white_list
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31
            )";

        let row = sqlx::query(sql)
            .bind(&fingerprint.ua_version) // ua_version
            .bind(&fingerprint.ua) // ua
            .bind(&fingerprint.language_type) // language_type
            .bind(&fingerprint.languages) // languages
            .bind(&fingerprint.gmt) // gmt
            .bind(&fingerprint.geography) // geography
            .bind(&fingerprint.geo_tips) // geo_tips
            .bind(&fingerprint.geo_rule) // geo_rule
            .bind(&fingerprint.longitude) // longitude
            .bind(&fingerprint.latitude) // latitude
            .bind(&fingerprint.radius) // radius
            .bind(&fingerprint.height) // height
            .bind(&fingerprint.width) // width
            .bind(&fingerprint.fonts_type) // fonts_type
            .bind(&fingerprint.fonts) // fonts
            .bind(&fingerprint.font_fingerprint) // font_fingerprint
            .bind(&fingerprint.web_rtc) // web_rtc
            .bind(&fingerprint.web_rtc_local_ip) // web_rtc_local_ip
            .bind(&fingerprint.canvas) // canvas
            .bind(&fingerprint.webgl) // webgl
            .bind(&fingerprint.hardware_acceleration) // hardware_acceleration
            .bind(&fingerprint.webgl_info) // webgl_info
            .bind(&fingerprint.audio_context) // audio_context
            .bind(&fingerprint.speech_voices) // speech_voices
            .bind(&fingerprint.media) // media
            .bind(&fingerprint.cpu) // cpu
            .bind(&fingerprint.memory) // memory
            .bind(&fingerprint.do_not_track) // do_not_track
            .bind(&fingerprint.battery) // battery
            .bind(&fingerprint.port_scan) // port_scan
            .bind(&fingerprint.white_list) // white_list
            .execute(get_db()?) // execute the query
            .await?;

        Ok(row.rows_affected() == 1)
    }

    /// fingerprints表删除数据
    pub async fn delete(id: i64) -> Result<bool, ApplicationServerError> {
        let sql = "DELETE FROM fingerprints WHERE id = ?1";

        let row = sqlx::query(sql)
            .bind(id) // fingerprint_id: 要删除的指纹 ID
            .execute(get_db()?) // 执行查询
            .await?;

        Ok(row.rows_affected() == 1) // 如果删除了 1 行则返回 true
    }

    /// fingerprints表查询指定id数据
    pub async fn query_fingerprint_by_id(id: i64) -> Result<Fingerprint, ApplicationServerError> {
        let pool = get_db()?;
        let fingerprint: Fingerprint = sqlx::query_as("select * from fingerprints where id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(fingerprint)
    }

    /// default
    pub async fn default_fingerprint() -> Result<Fingerprint, ApplicationServerError> {
        let pool = get_db()?;
        let fingerprint: Fingerprint = sqlx::query_as("select * from fingerprints where id = 1")
            .fetch_one(pool)
            .await?;

        Ok(fingerprint)
    }

    /// fingerprints表查询所有数据
    pub async fn query_fingerprints(
        payload: &PageParam,
    ) -> Result<(i64, Vec<Fingerprint>), ApplicationServerError> {
        let db = get_db()?;
        let mut page_num = payload.page_num.unwrap_or_else(|| 0);
        let page_size = payload.page_size.unwrap_or_else(|| 10);
        let (total,): (i64,) = sqlx::query_as("select count(1) from fingerprints")
            .fetch_one(db)
            .await?;
        if page_num <= 0 || ((page_num * page_size) as i64) > total {
            page_num = 0
        }
        let offset = page_num * page_size;

        let fingerprints: Vec<Fingerprint> =
            sqlx::query_as("select * from fingerprints limit $1 offset $2")
                .bind(page_size)
                .bind(offset)
                .fetch_all(get_db()?)
                .await?;

        Ok((total, fingerprints))
    }

    /// fingerprints表查询所有数据
    /// 更新数据 由于更新的数据有好几个 为了简单索性更新全部数据 (根据id来更新数据)
    pub async fn update_fingerprints(
        fingerprint: Fingerprint,
    ) -> Result<bool, ApplicationServerError> {
        let sql = "
            UPDATE fingerprints
            SET 
                ua_version = ?1,
                ua = ?2,
                language_type = ?3,
                languages = ?4,
                gmt = ?5,
                geography = ?6,
                geo_tips = ?7,
                geo_rule = ?8,
                longitude = ?9,
                latitude = ?10,
                radius = ?11,
                height = ?12,
                width = ?13,
                fonts_type = ?14,
                fonts = ?15,
                font_fingerprint = ?16,
                web_rtc = ?17,
                web_rtc_local_ip = ?18,
                canvas = ?19,
                webgl = ?20,
                hardware_acceleration = ?21,
                webgl_info = ?22,
                audio_context = ?23,
                speech_voices = ?24,
                media = ?25,
                cpu = ?26,
                memory = ?27,
                do_not_track = ?28,
                battery = ?29,
                port_scan = ?30,
                white_list = ?31,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?32;
        ";

        let row = sqlx::query(sql)
            .bind(&fingerprint.ua_version)
            .bind(&fingerprint.ua)
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
            .bind(fingerprint.id) // 绑定更新的 id
            .execute(get_db()?) // 执行更新
            .await?;

        Ok(row.rows_affected() == 1)
    }
}
