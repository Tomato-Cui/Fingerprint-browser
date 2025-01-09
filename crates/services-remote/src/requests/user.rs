use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn logout() -> Result<JsonRespnse, anyhow::Error> {
    let json_response: JsonRespnse = client::REQUEST
        .get(client::Client::build_url("/users/logout")?)
        .await?;

    Ok(json_response)
}

pub async fn login(nickname: &str, password: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
            "username": nickname,
            "password": password
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/users/login")?, &data)
        .await?;

    if let Some(data) = &json_response.data {
        if let Some(token) = data.get("token") {
            let token_str = token.to_string();
            let token_str = token_str.replace("\\", "").replace("\"", "");
            states::auth::set_token(&token_str).await;
        }
    }

    Ok(json_response)
}

pub async fn regsiter(
    email: &str,
    code: &str,
    nickname: &str,
    password: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "email": email,
        "code": code,
        "username": nickname,
        "password": password,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/users/register")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn query_search_by_email(email: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "email": email,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/users/search-by-email")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn reset_password(
    email: &str,
    password1: &str,
    password2: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "email": email,
        "password1": password1,
        "password2": password2,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/users/reset-password")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn register_send(email: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "email": email,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/users/register/send")?, &data)
        .await?;

    Ok(json_response)
}
