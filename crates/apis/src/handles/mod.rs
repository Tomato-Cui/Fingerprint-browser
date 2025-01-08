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

pub fn success_message(message: &str) -> Option<String> {
    Some(message.to_string())
}

pub fn warn_message<T: ToString>(error: T) -> Option<String> {
    Some(format!("操作失败: {}", error.to_string()))
}
