use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryByGroupPayload {
    pub proxy_group_id: u32,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct BatchDeletePayload {
    pub ids: Vec<u32>,
}

#[derive(Deserialize)]
pub struct IdPayload {
    pub id: u32,
}
