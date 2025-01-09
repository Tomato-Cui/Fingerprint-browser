use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_uuid(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-transfer-historys/query/id")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn query(page_num: u32, page_size: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-transfer-historys/query")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn batch_create(
    environment_uuids: Vec<String>,
    user_email: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuids": environment_uuids,
        "user_email": user_email,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-transfer-historys/batch")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn delete_by_uuid(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environmnet-transfer-historys")?,
            &data,
        )
        .await?;

    Ok(json_response)
}
