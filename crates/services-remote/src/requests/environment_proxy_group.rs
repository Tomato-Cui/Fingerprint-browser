use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-proxy-groups/id")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn query(page_num: u32, page_size: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-proxy-groups")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn create(
    proxy_group: models::environment_proxy_group::ProxyGroup,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-proxy-groups")?,
            &proxy_group,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("创建失败: {}", e))?;

    Ok(json_response)
}

pub async fn modify(
    proxy_group: models::environment_proxy_group::ProxyGroup,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environmnet-proxy-groups")?,
            &proxy_group,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("更新失败: {}", e))?;

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
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("删除失败: {}", e))?;

    Ok(json_response)
}
