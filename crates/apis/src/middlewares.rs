pub mod auth;
pub mod logger;

#[derive(Clone)]
pub struct CurrentUser {
    pub user_uuid: String,
}
