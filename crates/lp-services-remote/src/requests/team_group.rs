use lp_cores::requests::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/team-groups/id")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn query_all(team_id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_id": team_id,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/team-groups")?, &data)
        .await?;

    Ok(json_response)
}
