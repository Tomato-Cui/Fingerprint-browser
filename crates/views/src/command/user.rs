use crate::response::AppResponse;
use serde_json::{json, Value};

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
pub async fn login(account: &str, password: &str) -> Result<AppResponse<Value>, tauri::Error> {
    match services::user::login(account, password).await {
        Ok(v) => Ok(AppResponse::success(
            Some("登录成功".to_string()),
            Some(json!({
                "token": v
            })),
        )),
        Err(_) => Ok(AppResponse::<Value>::fail(Some("登录失败".to_string()))),
    }
}

#[tauri::command]
pub async fn register(
    email: &str,
    account: &str,
    password: &str,
) -> Result<AppResponse<bool>, tauri::Error> {
    match services::user::regsiter(email, account, password).await {
        Ok(v) => {
            if v {
                Ok(AppResponse::success(Some("注册成功".to_string()), Some(v)))
            } else {
                Ok(AppResponse::success(Some("注册失败".to_string()), Some(v)))
            }
        }
        Err(_) => Ok(AppResponse::<bool>::fail(Some("注册失败".to_string()))),
    }
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
pub async fn reset_password(
    email: &str,
    password1: &str,
    password2: &str,
) -> Result<AppResponse<bool>, tauri::Error> {
    let (success_msg, warn_msg) = (Some("重置密码成功".to_string()), |v| {
        Some(format!("重置密码失败: {}", v))
    });

    match services::user::reset_password(email, password1, password2).await {
        Ok(ok) => {
            if ok {
                Ok(AppResponse::<bool>::success(success_msg, Some(ok)))
            } else {
                Ok(AppResponse::<bool>::fail(warn_msg("未知错误".to_string())))
            }
        }
        Err(r) => Ok(AppResponse::<bool>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn logout() -> Result<AppResponse<bool>, tauri::Error> {
    match services::user::logout().await {
        Ok(v) => {
            if v {
                Ok(AppResponse::success(Some("退出成功".to_string()), Some(v)))
            } else {
                Ok(AppResponse::success(Some("退出失败".to_string()), Some(v)))
            }
        }
        Err(_) => Ok(AppResponse::<bool>::fail(Some("退出失败".to_string()))),
    }
}