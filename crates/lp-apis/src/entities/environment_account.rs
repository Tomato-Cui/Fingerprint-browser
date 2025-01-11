use serde::Deserialize;

#[derive(Deserialize)]
pub struct BatchDeletePayload {
    pub ids: Vec<u32>,
}

#[derive(Deserialize)]
pub struct QueryByIdPayload {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct DeleteByIdPayload {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct QueryByEnvironmentUUidPayload {
    pub environment_uuid: String,
    pub page_num: u32,
    pub page_size: u32,
}
