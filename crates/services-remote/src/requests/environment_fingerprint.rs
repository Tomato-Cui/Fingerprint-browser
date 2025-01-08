use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-fingerprints/id")?,
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
            client::Client::build_url("/environment-fingerprints/query")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn create(
    fingerprint: models::environment_fingerprint::EnvironmentFingerprint,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-fingerprints")?,
            &fingerprint,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("创建失败: {}", e))?;

    Ok(json_response)
}

pub async fn modify(
    fingerprint: models::environment_fingerprint::EnvironmentFingerprint,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-fingerprints")?,
            &fingerprint,
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
            client::Client::build_url("/environment-fingerprints")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("删除失败: {}", e))?;

    Ok(json_response)
}
