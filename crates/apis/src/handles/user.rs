use crate::entities::user::*;
use crate::response::AppResponse;
use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn handle_login(Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    match services::user::login(&payload.username, &payload.password).await {
        Ok(token) => AppResponse::success(
            Some("登录成功".to_string()),
            Some(json!({ "token": token })),
        ),
        Err(e) => AppResponse::fail(Some(format!("登录失败: {}", e))),
    }
}

pub async fn handle_logout() -> impl IntoResponse {
    match services::user::logout().await {
        Ok(_) => AppResponse::<()>::success(Some("退出成功".to_string()), Some(())),
        Err(e) => AppResponse::fail(Some(format!("退出失败: {}", e))),
    }
}

pub async fn handle_register(Json(payload): Json<RegisterPayload>) -> impl IntoResponse {
    match services::user::regsiter(
        &payload.email,
        &payload.code,
        &payload.username,
        &payload.password,
    )
    .await
    {
        Ok(_) => AppResponse::success(Some("注册成功".to_string()), Some(())),
        Err(e) => AppResponse::fail(Some(format!("注册失败: {}", e))),
    }
}

pub async fn handle_search_by_email(
    Json(payload): Json<SearchByEmailPayload>,
) -> impl IntoResponse {
    match services::user::query_search_by_email(&payload.email).await {
        Ok(data) => AppResponse::success(Some("搜索成功".to_string()), Some(data)),
        Err(e) => AppResponse::fail(Some(format!("搜索失败: {}", e))),
    }
}

pub async fn handle_reset_password(Json(payload): Json<ResetPasswordPayload>) -> impl IntoResponse {
    match services::user::reset_password(&payload.email, &payload.password1, &payload.password2)
        .await
    {
        Ok(_) => AppResponse::success(Some("重置密码成功".to_string()), Some(())),
        Err(e) => AppResponse::fail(Some(format!("重置密码失败: {}", e))),
    }
}

pub async fn handle_register_send(Json(payload): Json<RegisterSendPayload>) -> impl IntoResponse {
    match services::user::register_send(&payload.email).await {
        Ok(_) => AppResponse::success(Some("发送成功".to_string()), Some(true)),
        Err(e) => AppResponse::fail(Some(format!("发送失败: {}", e))),
    }
}
