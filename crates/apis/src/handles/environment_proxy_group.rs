use super::{success_message, warn_message};
use crate::entities::{IdPayload, Pagination};
use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use models::environment_proxy_group::ProxyGroup;
use serde_json::Value;

pub async fn query_by_id(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match services::environment_proxy_group::query_by_group_id(payload.id).await {
        Ok(data) => AppResponse::<ProxyGroup>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<ProxyGroup>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn query(state: Extension<CurrentUser>, payload: Query<Pagination>) -> impl IntoResponse {
    match services::environment_proxy_group::query(
        &state.user_uuid,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn create(
    state: Extension<CurrentUser>,
    Json(payload): Json<ProxyGroup>,
) -> impl IntoResponse {
    match services::environment_proxy_group::create(&state.user_uuid, payload).await {
        Ok(data) => AppResponse::<bool>::success(success_message("创建成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("创建失败: {}", r))),
    }
}

pub async fn modify(Json(payload): Json<ProxyGroup>) -> impl IntoResponse {
    if let None = payload.id {
        return AppResponse::<bool>::success(success_message("更新失败"), Some(false));
    }

    match services::environment_proxy_group::update(payload.id.unwrap_or_default() as u32, payload)
        .await
    {
        Ok(data) => AppResponse::<bool>::success(success_message("更新成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn delete(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match services::environment_proxy_group::delete(payload.id).await {
        Ok(data) => AppResponse::<bool>::success(success_message("删除成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}
