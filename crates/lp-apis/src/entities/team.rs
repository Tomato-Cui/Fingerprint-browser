use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct TeamQueryPayload {
    pub team_id: u32,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct TeamSearchByNamePayload {
    pub team_name: String,
}

#[derive(Deserialize)]
pub struct TeamGroupQueryPayload {
    pub team_id: u32,
    pub team_group_id: u32,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct BlockedPayload {
    pub current_user_uuid: String,
    pub team_id: u32,
}

#[derive(Deserialize)]
pub struct CreateTeamPayload {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ModifyTeamPayload {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ModifyTeamUserInfoPayload {
    pub team_id: u32,
    pub description: Option<String>,
    pub team_group_id: u32,
    pub current_user_uuid: String,
}
