use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/teams/query/id")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn is_leader(team_id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": team_id
    });
    let json_response = client::REQUEST
        .post(
            client::Client::build_url(&format!("/teams/query/is-leader",))?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn query_search_by_name(team_name: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_name": team_name
    });
    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/teams/query/search-by-name")?,
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
        .post(client::Client::build_url("/teams/query")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn query_current_team() -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .get(client::Client::build_url("/teams/query/current")?)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn query_team_all_user(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_id": team_id,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/teams/query/all-user")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn query_team_all_blocked_user(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_id": team_id,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/teams/query/all-blocked-user")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn query_team_group_all_user(
    team_id: u32,
    team_group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_id": team_id,
        "team_group_id": team_group_id,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/teams/query/group/all-user")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn blocked(current_user_uuid: &str, team_id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "current_user_uuid": current_user_uuid,
        "team_id": team_id,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/teams/blocked")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("拉黑失败: {}", e))?;

    Ok(json_response)
}

pub async fn unblocked(
    current_user_uuid: &str,
    team_id: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "current_user_uuid": current_user_uuid,
        "team_id": team_id,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/teams/un-blocked")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("取消拉黑失败: {}", e))?;

    Ok(json_response)
}

pub async fn create(name: &str, description: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "name": name,
        "description": description,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/teams/create")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("创建失败: {}", e))?;

    Ok(json_response)
}

pub async fn modify(id: u32, name: &str, description: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
        "name": name,
        "description": description,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/teams")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("更新失败: {}", e))?;

    Ok(json_response)
}

pub async fn switch_team(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });
    let json_response = client::REQUEST
        .put(client::Client::build_url(&format!("/teams/switch"))?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("切换失败: {}", e))?;

    Ok(json_response)
}

pub async fn remove_current_user(
    team_id: u32,
    current_user_uuid: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_id": team_id,
        "current_user_uuid": current_user_uuid,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/teams/remove-user")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("移除失败: {}", e))?;

    Ok(json_response)
}

pub async fn modify_team_user_info(
    team_id: u32,
    description: Option<String>,
    team_group_id: u32,
    current_user_uuid: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_id": team_id,
        "description": description,
        "team_group_id": team_group_id,
        "current_user_uuid": current_user_uuid,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/teams/user-info")?, &data)
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
        .delete(client::Client::build_url("/teams")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("删除失败: {}", e))?;

    Ok(json_response)
}
