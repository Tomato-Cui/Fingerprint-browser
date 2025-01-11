use lp_models::extension::Extension;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TeamCreatePayload {
    pub team_id: String,
    pub extension_uuid: String,
}

#[derive(Deserialize)]
pub struct QueryByTeamPayload {
    pub team_id: u32,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct QueryByEnvironmentUuidPayload {
    pub environment_uuid: String,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct EnvironmentUseExtensionPayload {
    pub extension_uuid: String,
    pub environment_uuids: Vec<String>,
}

#[derive(Deserialize)]
pub struct EnvironmentRemoveExtensionPayload {
    pub extension_uuid: String,
    pub environment_uuid: String,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    pub extension_uuid: String,
    pub extension: Extension,
}

#[derive(Deserialize)]
pub struct UserToggleExtensionPayload {
    pub extension_uuid: String,
    pub open: bool,
}

#[derive(Deserialize)]
pub struct ExtensionUUidPayload {
    pub extension_uuid: String,
}
