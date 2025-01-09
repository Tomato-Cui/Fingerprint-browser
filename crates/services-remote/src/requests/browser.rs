use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn start(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/browsers/start")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn starts(environment_uuids: Vec<String>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuids": environment_uuids,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/browsers/starts")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn stops(environment_uuids: Vec<String>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuids": environment_uuids,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/browsers/stops")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn stop_all() -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(client::Client::build_url("/browsers/stop-all")?, &json!({}))
        .await?;

    Ok(json_response)
}

pub async fn status() -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .get(client::Client::build_url("/browsers/status")?)
        .await?;

    Ok(json_response)
}
