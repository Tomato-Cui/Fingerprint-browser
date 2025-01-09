use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_uuid(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/query/uuid")?,
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
        .post(client::Client::build_url("/environments/query")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn query_by_group(
    group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "group_id": group_id,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/query/group")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn query_by_team(
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
            client::Client::build_url("/environments/query/team")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn query_by_extension(
    extension_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "extension_uuid": extension_uuid,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/query/extension")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn create(name: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "name": name,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/environments/create")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn detail_create(
    environment_info: models::environment::EnvironmentInfo,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/detail/create")?,
            &environment_info,
        )
        .await?;

    Ok(json_response)
}

pub async fn batch_create(names: Vec<String>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "names": names,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/environments/batch")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn modify_info(
    environment_info: models::environment::EnvironmentInfo,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url(&format!("/environments"))?,
            &environment_info,
        )
        .await?;

    Ok(json_response)
}

pub async fn modify_proxy(
    environment_uuid: &str,
    proxy: models::environment_proxies::Proxy,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
        "porxy": proxy,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/environments/proxy")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn modify_basic_info(
    environment: models::environment::Environment,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environments/basic")?,
            &environment,
        )
        .await?;

    Ok(json_response)
}

pub async fn move_to_group(
    environment_uuid: &str,
    group_id: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
        "group_id": group_id,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environments/move-to-group")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn batch_move_to_group(
    environment_ids: Vec<String>,
    group_id: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_ids": environment_ids,
        "group_id": group_id,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environments/batch/move-to-group")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn delete(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .delete(client::Client::build_url("/environments")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn batch_delete(environment_uuids: Vec<String>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuids": environment_uuids,
    });

    let json_response = client::REQUEST
        .delete(client::Client::build_url("/environments/batch")?, &data)
        .await?;

    Ok(json_response)
}
