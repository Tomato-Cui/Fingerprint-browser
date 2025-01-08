use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct TeamReceiveQueryPayload {
    pub team_id: u32,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct TeamSendPayload {
    pub team_id: u32,
    pub user_uuid: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UserSendPayload {
    pub team_name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct TeamAllowPayload {
    pub id: u32,
    pub user_uuid: String,
    pub team_id: u32,
}

#[derive(Deserialize)]
pub struct UserAllowPayload {
    pub id: u32,
    pub team_id: u32,
}
