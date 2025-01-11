use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePayload {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct ModifyPayload {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct IdPayload {
    pub id: u32,
}
