use cores::request::{client, JsonRespnse};
use serde_json::{json, Value};

pub async fn logout() -> () {
    states::auth::clear_token().await;
}

pub async fn login(username: &str, password: &str) -> Result<Value, anyhow::Error> {
    let data = json!({
            "username": username,
            "password": password
    });

    let response = client::REQUEST
        .post(client::Client::build_url("/api/auth/login")?, &data)
        .await?;

    let json_response: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("登录失败: {}", e))?;

    if let Some(Value::Object(obj)) = &json_response.data {
        if let Some(Value::String(token)) = obj.get("token") {
            states::auth::set_token(token).await;
            return Ok(json!({"token": token}));
        }
    }

    return Err(anyhow::anyhow!("login failed"));
}

pub async fn register(
    username: &str,
    password: &str,
    email: &str,
    captcha: &str,
) -> Result<(), anyhow::Error> {
    let data = json!({
        "account": {
            "email": email,
            "username": username,
            "password": password
        },
        "captcha": captcha
    });

    let response = client::REQUEST
        .post(client::Client::build_url("/api/auth/register")?, &data)
        .await
        .map_err(|e| anyhow::anyhow!("注册失败: {}", e))?;

    let _: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("注册失败: {}", e))?;

    Ok(())
}

pub async fn send(email: &str) -> Result<(), anyhow::Error> {
    let data = json!({
            "email": email,
    });

    let response = client::REQUEST
        .post(client::Client::build_url("/api/auth/send")?, &data)
        .await
        .map_err(|e| anyhow::anyhow!("验证码发送失败: {}", e))?;

    let _: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("验证码发送失败: {}", e))?;

    Ok(())
}
