use lp_cores::requests::{client, JsonRespnse as JsonResponse};
use serde_json::json;

/// 根据 ID 查询环境标签
pub async fn query_by_id(id: u32) -> Result<JsonResponse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-tags/query/id")?, // 修改路径：environment-groups -> environment-tags
            &data,
        )
        .await?;

    Ok(json_response)
}

/// 分页查询环境标签
pub async fn query(page_num: u32, page_size: u32) -> Result<JsonResponse, anyhow::Error> {
    let data = json!({
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-tags/query")?, // 修改路径：environment-groups -> environment-tags
            &data,
        )
        .await?;

    Ok(json_response)
}

/// 创建环境标签
pub async fn create(
    name: &str,
    description: Option<String>,
) -> Result<JsonResponse, anyhow::Error> {
    let data = json!({
        "name": name,
        "description": description,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/environment-tags")?, &data) // 修改路径：environment-groups -> environment-tags
        .await?;

    Ok(json_response)
}

/// 修改环境标签
pub async fn modify(
    id: u32,
    name: &str,
    description: Option<String>,
) -> Result<JsonResponse, anyhow::Error> {
    let data = json!({
        "id": id,
        "name": name,
        "description": description,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/environment-tags")?, &data) // 修改路径：environment-groups -> environment-tags
        .await?;

    Ok(json_response)
}

/// 删除环境标签
pub async fn delete(id: u32) -> Result<JsonResponse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .delete(client::Client::build_url("/environment-tags")?, &data) // 修改路径：environment-groups -> environment-tags
        .await?;

    Ok(json_response)
}
