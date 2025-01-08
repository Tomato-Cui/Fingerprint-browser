use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct QueryByGroupIdPayload {
    pub group_id: u32,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct QueryByTeamIdPayload {
    pub team_id: u32,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct QueryByExtensionUuidPayload {
    pub extension_uuid: String,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    pub name: String,
}

#[derive(Deserialize)]
pub struct BatchCreatePayload {
    pub names: Vec<String>,
}

#[derive(Deserialize)]
pub struct ModifyProxy {
    pub environment_uuid: String,
    pub porxy: models::environment_proxies::Proxy,
}

#[derive(Deserialize)]
pub struct MoveToGroupPayload {
    pub environment_uuid: String,
    pub group_id: u32,
}

#[derive(Deserialize)]
pub struct BatchMoveToGroupPayload {
    pub environment_ids: Vec<String>,
    pub group_id: u32,
}

#[derive(Deserialize)]
pub struct BatchDeletePayload {
    pub environment_uuids: Vec<String>,
}
