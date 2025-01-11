use lp_cores::requests::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-proxy-groups/query/id")?,
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
            client::Client::build_url("/environmnet-proxy-groups/query")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn create(
    proxy_group: lp_models::environment_proxy_group::ProxyGroup,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-proxy-groups")?,
            &proxy_group,
        )
        .await?;

    Ok(json_response)
}

pub async fn modify(
    proxy_group: lp_models::environment_proxy_group::ProxyGroup,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environmnet-proxy-groups")?,
            &proxy_group,
        )
        .await?;

    Ok(json_response)
}

pub async fn delete(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environmnet-proxy-groups")?,
            &data,
        )
        .await?;

    Ok(json_response)
}
