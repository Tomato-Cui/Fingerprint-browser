use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInfoWithGroup {
    nickname: String,
    email: String,
    description: String,
    group_name: Option<String>,
}
