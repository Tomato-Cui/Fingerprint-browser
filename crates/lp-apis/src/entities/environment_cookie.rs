use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryByEnvironmentUuidPayload {
    pub environment_uuid: String,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    pub environment_uuid: String,
    pub cookie_str: String,
}

#[derive(Deserialize)]
pub struct ModifyPayload {
    pub environment_uuid: String,
    pub cookie_str: String,
}

#[derive(Deserialize)]
pub struct DeletePayload {
    pub environment_uuid: String,
}
