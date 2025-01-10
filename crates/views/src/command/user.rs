use crate::response::AppResponse;
use cores::request::JsonRespnse;

#[tauri::command]
pub async fn login(account: &str, password: &str) -> Result<JsonRespnse, tauri::Error> {
    let res = services_remote::requests::user::login(account, password).await?;
    if let Some(data) = &res.data {
        if let Some(token) = data.get("token") {
            let token = token.to_string();
            let token_str = token.replace("\\", "").replace("\"", "");
            states::auth::set_token(&token_str).await;
        }
    }

    Ok(res)
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
    Ok(services_remote::requests::user::query_search_by_email(email).await?)
}

#[tauri::command]
pub async fn reset_password(
    email: &str,
    code: &str,
    password: &str,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::user::reset_password(email, code, password).await?)
}

#[tauri::command]
pub async fn logout() -> Result<AppResponse<bool>, tauri::Error> {
    states::auth::clear_token().await;
    Ok(AppResponse::success(
        Some("退出成功".to_string()),
        Some(true),
    ))
}

#[tauri::command]
pub async fn register_send(email: &str) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::user::register_send(email).await?)
}

#[tauri::command]
pub async fn reset_password_send(email: &str) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::user::reset_password_send(email).await?)
}
