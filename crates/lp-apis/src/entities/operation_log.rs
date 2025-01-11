use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryByTeamIdPayload {
    pub id: u32,
    pub page_num: u32,
    pub page_size: u32,
}
