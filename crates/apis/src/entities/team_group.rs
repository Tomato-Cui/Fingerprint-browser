use serde::Deserialize;

#[derive(Deserialize)]
pub struct TeamGroupQueryPayload {
    pub team_id: u32,
}
