use serde::Deserialize;

#[derive(Deserialize)]
pub struct RecoversPayload {
    pub environment_uuids: Vec<String>,
}

#[derive(Deserialize)]
pub struct DeleteBatchPayload {
    pub environment_uuids: Vec<String>,
}
