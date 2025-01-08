pub mod browser;
pub mod environment;
pub mod environment_account;
pub mod environment_cookie;
pub mod environment_fingerprint;
pub mod environment_group;
pub mod environment_proxies;
pub mod environment_proxy_group;
pub mod environment_transfer_history;
pub mod environment_trash;
pub mod extension;
pub mod message;
pub mod team;
pub mod team_group;
pub mod user;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct IdPayload {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct EnvironmentUUIdPayload {
    pub environment_uuid: String,
}
