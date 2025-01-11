use lp_cores::requests::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-proxies/query/id")?,
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
            client::Client::build_url("/environment-proxies/query")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn query_by_group(
    proxy_group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "proxy_group_id": proxy_group_id,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-proxies/query/group")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn create(
    proxy: lp_models::environment_proxies::Proxy,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(client::Client::build_url("/environment-proxies")?, &proxy)
        .await?;

    Ok(json_response)
}

pub async fn modify(
    proxy: lp_models::environment_proxies::Proxy,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(client::Client::build_url("/environment-proxies")?, &proxy)
        .await?;

    Ok(json_response)
}

pub async fn delete(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .delete(client::Client::build_url("/environment-proxies")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn batch_delete(ids: Vec<u32>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "ids": ids,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environment-proxies/batch")?,
            &data,
        )
        .await?;

    Ok(json_response)
}
