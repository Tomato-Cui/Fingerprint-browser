use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdPayload {
    pub id: u32,
}
