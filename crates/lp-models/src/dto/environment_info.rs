use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct EnvironmentDetailWithResponse {
    pub id: i32,
    pub uuid: String,
    pub user_uuid: Option<String>,
    pub team_id: Option<i32>,
    pub proxy_id: Option<i32>,
    pub fp_info_id: Option<i32>,
    pub group_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub default_urls: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub lasted_at: Option<String>,
    pub deleted_at: Option<String>,

    pub fp_id: Option<i32>,
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
    pub fp_created_at: Option<String>,
    pub fp_updated_at: Option<String>,
    pub fp_deleted_at: Option<String>,

    // Fields from Proxy
    pub proxy_kind: Option<String>,
    pub proxy_host: Option<String>,
    pub proxy_port: Option<String>,
    pub proxy_username: Option<String>,
    pub proxy_password: Option<String>,
    pub proxy_user_uuid: Option<String>,
    pub proxy_environment_group_id: Option<i32>,
    pub proxy_created_at: Option<String>,
    pub proxy_updated_at: Option<String>,
    pub proxy_deleted_at: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct EnvironmentDetailWithAdvanceCreateRequest {
    pub uuid: Option<String>,
    pub team_id: Option<i32>,
    pub proxy_id: Option<i32>,
    pub fp_info_id: Option<i32>,
    pub group_id: Option<i32>,
    pub tag_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub default_urls: Option<String>,

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

    pub proxy_kind: Option<String>,
    pub proxy_host: Option<String>,
    pub proxy_port: Option<String>,
    pub proxy_username: Option<String>,
    pub proxy_password: Option<String>,
    pub proxy_user_uuid: Option<String>,
    pub proxy_environment_group_id: Option<i32>,
}
