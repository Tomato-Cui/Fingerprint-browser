use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_uuid(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environmnet-transfer-historys/id")?,
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
            client::Client::build_url("/environmnet-transfer-historys")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

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
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("创建失败: {}", e))?;

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
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("删除失败: {}", e))?;

    Ok(json_response)
}
