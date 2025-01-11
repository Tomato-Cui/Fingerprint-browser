use lp_cores::requests::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-groups/query/id")?,
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
            client::Client::build_url("/environmnet-groups/query")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn create(name: &str, description: Option<String>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "name": name,
        "description": description,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/environmnet-groups")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn modify(
    id: u32,
    name: &str,
    description: Option<String>,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
        "name": name,
        "description": description,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/environmnet-groups")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn delete(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .delete(client::Client::build_url("/environmnet-groups")?, &data)
        .await?;

    Ok(json_response)
}
