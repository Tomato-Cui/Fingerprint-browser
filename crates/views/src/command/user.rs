use crate::response::AppResponse;
use cores::request::JsonRespnse;

pub async fn get_user_id() -> Result<String, anyhow::Error> {
    let token = states::auth::get_token().await;
    if let Some(token_str) = token {
        match commons::encryption::verify_token(&token_str) {
            Ok(user_uuid) => Ok(user_uuid),
            Err(_e) => Err(anyhow::anyhow!("token 异常")),
        }
    } else {
        Err(anyhow::anyhow!("用户处于退出状态"))
    }
}

#[tauri::command]
pub async fn login(account: &str, password: &str) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::user::login(account, password).await?)
}

#[tauri::command]
pub async fn register(
    email: &str,
    code: &str,
    account: &str,
    password: &str,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::user::regsiter(email, code, account, password).await?)
}

#[tauri::command]
pub async fn is_login() -> Result<AppResponse<bool>, tauri::Error> {
    let token = states::auth::get_token().await;
    if let Some(token_str) = token {
        match commons::encryption::verify_token(&token_str) {
            Ok(_id) => Ok(AppResponse::success(
                Some("用户处于登录状态".to_string()),
                Some(true),
            )),
            Err(_) => Ok(AppResponse::<bool>::fail(Some(
                "用户处于退出状态".to_string(),
            ))),
        }
    } else {
        Ok(AppResponse::<bool>::fail(Some(
            "用户处于退出状态".to_string(),
        )))
    }
}

#[tauri::command]
pub async fn user_query_search_by_email(email: &str) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::user::query_search_by_email(email).await?)
}

#[tauri::command]
pub async fn reset_password(
    email: &str,
    password1: &str,
    password2: &str,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::user::reset_password(email, password1, password2).await?)
}

#[tauri::command]
pub async fn logout() -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::user::logout().await?)
}

#[tauri::command]
pub async fn register_send(email: &str) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::user::register_send(email).await?)
}
