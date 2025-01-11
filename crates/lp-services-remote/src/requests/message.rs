use lp_cores::requests::{client, JsonRespnse};
use serde_json::json;

pub async fn user_receive_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/messages/user/receive-query")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn team_receive_query(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_id": team_id,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/messages/team/receive-query")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn team_send(
    user_uuid: &str,
    team_id: u32,
    description: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "user_uuid": user_uuid,
        "team_id": team_id,
        "description": description,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/messages/team/send")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn user_send(team_name: &str, description: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_name": team_name,
        "description": description,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/messages/user/send")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn reject(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/messages/reject")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn team_allow(
    id: u32,
    user_uuid: &str,
    team_id: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
        "user_uuid": user_uuid,
        "team_id": team_id,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/messages/team/allow")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn user_allow(id: u32, team_id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
        "team_id": team_id,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/messages/user/allow")?, &data)
        .await?;

    Ok(json_response)
}
