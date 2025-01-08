use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_uuid(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-cookies/environment/id")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn create(
    environment_uuid: &str,
    cookie_str: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
        "cookie_str": cookie_str,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-cookies/environment")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("创建失败: {}", e))?;

    Ok(json_response)
}

pub async fn modify(
    environment_uuid: &str,
    cookie_str: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
        "cookie_str": cookie_str,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-cookies/environment")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("更新失败: {}", e))?;

    Ok(json_response)
}

pub async fn delete(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environment-cookies/environment")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("删除失败: {}", e))?;

    Ok(json_response)
}
