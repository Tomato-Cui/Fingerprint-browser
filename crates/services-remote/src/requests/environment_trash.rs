use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_uuid(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/environment-trash/query/uuid")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn query(page_num: u32, page_size: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/environment-trash/query")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn recover(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-trash/recover")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn recovers(environment_uuids: Vec<String>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuids": environment_uuids,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-trash/recovers")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn recover_all() -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-trash/recover-all")?,
            &json!({}),
        )
        .await?;

    Ok(json_response)
}

pub async fn delete_batch(environment_uuids: Vec<String>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuids": environment_uuids,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environment-trash/batch")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn clean() -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environment-trash/clean")?,
            &json!({}),
        )
        .await?;

    Ok(json_response)
}
