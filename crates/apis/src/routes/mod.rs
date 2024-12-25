// pub mod browser;
// pub mod environment;
// pub mod environment_fingerprint;
// pub mod environment_group;
// pub mod environment_proxies;
// pub mod environment_trash;
// pub mod team;
// pub mod user;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    page_num: u32,
    page_size: u32,
}
