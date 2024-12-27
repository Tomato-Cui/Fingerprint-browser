use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInfoWithGroup {
    user_uuid: String,
    nickname: String,
    email: String,
    description: String,
    group_name: Option<String>,
}
