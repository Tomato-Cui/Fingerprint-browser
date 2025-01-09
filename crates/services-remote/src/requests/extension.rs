use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn create(extension: models::extension::Extension) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(client::Client::build_url("/extensions/create")?, &extension)
        .await?;

    Ok(json_response)
}

pub async fn user_create(extension_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "extension_uuid": extension_uuid,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/extensions/user/create")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn team_create(
    team_id: &str,
    extension_uuid: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_id": team_id,
        "extension_uuid": extension_uuid,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/extensions/team/create")?, &data)
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
        .post(client::Client::build_url("/extensions/query/team")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn query_by_user(page_num: u32, page_size: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/extensions/query/user")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn query_by_environment(
    environment_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/extensions/query/environment")?,
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
        .post(client::Client::build_url("/extensions/query")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn environment_use_extension(
    extension_uuid: &str,
    environment_uuids: Vec<String>,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "extension_uuid": extension_uuid,
        "environment_uuids": environment_uuids,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/extensions/environmnet/use")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn environment_remove_extension(
    extension_uuid: &str,
    environment_uuid: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "extension_uuid": extension_uuid,
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/extensions/environmnet/remove")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn update(
    extension_uuid: &str,
    extension: models::extension::Extension,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "extension_uuid": extension_uuid,
        "extension": extension,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/extensions/update")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn user_toggle_extension(
    extension_uuid: &str,
    open: bool,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "extension_uuid": extension_uuid,
        "open": open,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/extensions/user/toggle-extension")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn delete_by_uuid(extension_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "extension_uuid": extension_uuid,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url(&format!("/extensions/delete/uuid",))?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn remove_by_user_uuid(extension_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "extension_uuid": extension_uuid,
    });
    let json_response = client::REQUEST
        .delete(
            client::Client::build_url(&format!("/extensions/remove/user-uuid"))?,
            &data,
        )
        .await?;

    Ok(json_response)
}
