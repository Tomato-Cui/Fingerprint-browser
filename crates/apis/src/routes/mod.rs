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
pub mod team;
pub mod team_group;
pub mod user;
pub mod user_team_temp;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    page_num: u32,
    page_size: u32,
}
