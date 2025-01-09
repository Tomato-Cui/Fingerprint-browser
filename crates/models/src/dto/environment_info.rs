use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct EnvironmentWithDetails {
    // Fields from Environment
    pub id: i32,
    pub uuid: String,
    pub user_uuid: String,
    pub team_id: i32,
    pub proxy_id: Option<i32>,
    pub fp_info_id: Option<i32>,
    pub group_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub default_urls: Option<String>,
    pub proxy_enable: i8,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub lasted_at: Option<String>,
    pub deleted_at: Option<String>,

    pub fp_id: Option<i32>,
    pub browser: Option<String>,
    pub ua: Option<String>,
    pub os: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub language_type: Option<i32>,
    pub languages: Option<String>,
    pub gmt: Option<String>,
    pub geography: Option<String>,
    pub geo_tips: Option<i32>,
    pub geo_rule: Option<i32>,
    pub longitude: Option<i32>,
    pub latitude: Option<i32>,
    pub radius: Option<i32>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub fonts_type: Option<i32>,
    pub fonts: Option<String>,
    pub font_fingerprint: Option<i32>,
    pub web_rtc: Option<i32>,
    pub web_rtc_local_ip: Option<String>,
    pub canvas: Option<i32>,
    pub webgl: Option<i32>,
    pub hardware_acceleration: Option<i32>,
    pub webgl_info: Option<i32>,
    pub audio_context: Option<i32>,
    pub speech_voices: Option<i32>,
    pub media: Option<i32>,
    pub cpu: Option<i32>,
    pub memory: Option<i32>,
    pub do_not_track: Option<i32>,
    pub battery: Option<i32>,
    pub port_scan: Option<i32>,
    pub white_list: Option<String>,
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

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct EnvironmentWithInfo {
    // Fields from Environment
    id: i32,
    uuid: String,
    user_uuid: String,
    team_id: i32,
    proxy_id: Option<i32>,
    fp_info_id: Option<i32>,
    group_id: Option<i32>,
    group_name: Option<String>,
    name: String,
    description: Option<String>,
    default_urls: Option<String>,
    proxy_enable: i8,
    created_at: Option<String>,
    updated_at: Option<String>,
    lasted_at: Option<String>,
    deleted_at: Option<String>,

    fp_id: Option<i32>,
    browser: Option<String>,
    ua: Option<String>,
    os: Option<String>,
    country: Option<String>,
    region: Option<String>,
    city: Option<String>,
    language_type: Option<i32>,
    languages: Option<String>,
    gmt: Option<String>,
    geography: Option<String>,
    geo_tips: Option<i32>,
    geo_rule: Option<i32>,
    longitude: Option<i32>,
    latitude: Option<i32>,
    radius: Option<i32>,
    height: Option<i32>,
    width: Option<i32>,
    fonts_type: Option<i32>,
    fonts: Option<String>,
    font_fingerprint: Option<i32>,
    web_rtc: Option<i32>,
    web_rtc_local_ip: Option<String>,
    canvas: Option<i32>,
    webgl: Option<i32>,
    hardware_acceleration: Option<i32>,
    webgl_info: Option<i32>,
    audio_context: Option<i32>,
    speech_voices: Option<i32>,
    media: Option<i32>,
    cpu: Option<i32>,
    memory: Option<i32>,
    do_not_track: Option<i32>,
    battery: Option<i32>,
    port_scan: Option<i32>,
    white_list: Option<String>,
    fp_created_at: Option<String>,
    fp_updated_at: Option<String>,
    fp_deleted_at: Option<String>,

    // Fields from Proxy
    proxy_kind: Option<String>,
    proxy_host: Option<String>,
    proxy_port: Option<String>,
    proxy_username: Option<String>,
    proxy_password: Option<String>,
    proxy_user_uuid: Option<String>,
    proxy_environment_group_id: Option<i32>,
    proxy_created_at: Option<String>,
    proxy_updated_at: Option<String>,
    proxy_deleted_at: Option<String>,
}
