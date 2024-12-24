pub mod browser;
pub mod environment;
pub mod environment_trash;
pub mod fingerprint;
pub mod group;
pub mod proxy;
pub mod team;
pub mod user;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    page_num: u32,
    page_size: u32,
}
