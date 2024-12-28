use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamGroupList {
    count:i32,
    id: i32,
    name: String,
    description: Option<String>,
    team_id: Option<i32>,
    team_group_permission_id: Option<i32>,
    created_at: Option<String>,
    updated_at: Option<String>,
    deleted_at: Option<String>,
}
