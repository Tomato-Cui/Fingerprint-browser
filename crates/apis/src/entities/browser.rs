use serde::Deserialize;

#[derive(Deserialize)]
pub struct StartPayload {
    pub environment_uuid: String,
}

#[derive(Deserialize)]
pub struct StartsPayload {
    pub environment_uuids: Vec<String>,
}

#[derive(Deserialize)]
pub struct StopsPayload {
    pub environment_uuids: Vec<String>,
}
