use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow, Default)]
pub struct UserExtensionInfo {
    pub id: Option<i32>,
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub release_url: Option<String>,
    pub size: Option<i32>,
    pub all_can_use: Option<i32>,
    pub open: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}
