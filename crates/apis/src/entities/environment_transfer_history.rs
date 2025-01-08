use serde::Deserialize;

#[derive(Deserialize)]
pub struct BatchCreatePayload {
    pub environment_uuids: Vec<String>,
    pub user_email: String,
}
