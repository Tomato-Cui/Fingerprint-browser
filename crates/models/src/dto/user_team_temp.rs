use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserTeamTempInfo {
    id: Option<i32>,             // 自增ID
    allow_1: Option<i32>,        // 允许1
    allow_2: Option<i32>,        // 允许2
    user_uuid: String,
    nickname: String,
    email: String,
    description: String,
    created_at: Option<String>,  // 创建时间
    updated_at: Option<String>,  // 更新时间
    deleted_at: Option<String>,  // 删除时间
}
