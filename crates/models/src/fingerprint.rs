use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Fingerprint {
    pub id: Option<i32>,                  // 自增ID
    pub owner_id: Option<i32>,            // 用户id
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
    #[allow(dead_code)]
    pub async fn insert(
        pool: &Pool<Sqlite>,
        user_id: u32,
        fingerprint: &Fingerprint,
    ) -> Result<bool, Error> {
        let sql = "
            INSERT INTO fingerprints (
                owner_id,ua,language_type,languages,gmt,geography,geo_tips,geo_rule,longitude,latitude,radius,height,
                width,fonts_type,fonts,font_fingerprint,web_rtc,web_rtc_local_ip,canvas,webgl,hardware_acceleration,
                webgl_info,audio_context,speech_voices,media,cpu,memory,do_not_track,battery,port_scan,white_list
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )";

        let row = sqlx::query(sql)
            .bind(user_id)
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
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_by_id(
        pool: &Pool<Sqlite>,
        user_id: u32,
        id: u32,
    ) -> Result<Fingerprint, Error> {
        let fingerprint: Fingerprint =
            sqlx::query_as("select * from fingerprints where id = ? and owner_id = ?")
                .bind(id)
                .bind(user_id)
                .fetch_one(pool)
                .await?;

        Ok(fingerprint)
    }

    #[allow(dead_code)]
    pub async fn default_fingerprint(pool: &Pool<Sqlite>) -> Result<Fingerprint, Error> {
        let fingerprint: Fingerprint = sqlx::query_as("select * from fingerprints where id = 1")
            .fetch_one(pool)
            .await?;

        Ok(fingerprint)
    }

    #[allow(dead_code)]
    pub async fn query_by_col(
        pool: &Pool<Sqlite>,
        user_id: u32,
        col_name: &str,
        col_value: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Fingerprint>), Error> {
        let (total,): (i64,) = if col_name.is_empty() {
            sqlx::query_as("select count(1) from fingerprints where owner_id = ?")
                .bind(user_id)
                .fetch_one(pool)
                .await?
        } else {
            sqlx::query_as(&format!(
                "select count(1) from fingerprints where {} = ? and owner_id = ?",
                col_name
            ))
            .bind(col_value)
            .bind(user_id)
            .fetch_one(pool)
            .await?
        };

        let offset = page_num * page_size;

        let fingerprints: Vec<Fingerprint> = if col_name.is_empty() {
            sqlx::query_as("select * from fingerprints where owner_id = ? limit ? offset ? ")
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else {
            sqlx::query_as(&format!(
                "select * from fingerprints where {} = ? and owner_id = ? limit ? offset ? ",
                col_name
            ))
            .bind(col_value)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok((total, fingerprints))
    }

    #[allow(dead_code)]
    pub async fn update(
        pool: &Pool<Sqlite>,
        user_id: u32,
        fingerprint: &Fingerprint,
    ) -> Result<bool, Error> {
        let sql = "
        UPDATE fingerprints
            SET ua = ?, language_type = ?, languages = ?, gmt = ?, geography = ?, geo_tips = ?, geo_rule = ?,
                longitude = ?, latitude = ?, radius = ?, height = ?, width = ?, fonts_type = ?, fonts = ?, font_fingerprint = ?, 
                web_rtc = ?, web_rtc_local_ip = ?, canvas = ?, webgl = ?, hardware_acceleration = ?, webgl_info = ?, audio_context = ?, 
                speech_voices = ?, media = ?, cpu = ?, memory = ?, do_not_track = ?, battery = ?, port_scan = ?, white_list = ?, 
                updated_at = CURRENT_TIMESTAMP
        WHERE id = ? and user_id = ?;
        ";

        let row = sqlx::query(sql)
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
            .bind(fingerprint.id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn update_by_col(
        pool: &Pool<Sqlite>,
        user_id: u32,
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
            "UPDATE fingerprints SET {} = ? WHERE id = ? and owner_id = ?",
            col_name
        ))
        .bind(col_value)
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, user_id: u32, id: u32) -> Result<bool, Error> {
        let sql = "DELETE FROM fingerprints WHERE id = ? and owner_id = ?";

        let row = sqlx::query(sql)
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1) // 如果删除了 1 行则返回 true
    }
}
