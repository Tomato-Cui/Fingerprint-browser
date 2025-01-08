use crate::entities::{environment_proxies::*, Pagination};
use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use models::environment_proxies::Proxy;
use serde_json::Value;

use super::{success_message, warn_message};

pub async fn query_by_id(
    state: Extension<CurrentUser>,
    Json(payload): Json<IdPayload>,
) -> impl IntoResponse {
    match services::environment_proxy::query_by_id(&state.user_uuid, payload.id).await {
        Ok(data) => AppResponse::<Proxy>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Proxy>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn query(state: Extension<CurrentUser>, payload: Query<Pagination>) -> impl IntoResponse {
    match services::environment_proxy::query(&state.user_uuid, payload.page_num, payload.page_size)
        .await
    {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn query_by_group(
    state: Extension<CurrentUser>,
    Json(payload): Json<QueryByGroupPayload>,
) -> impl IntoResponse {
    match services::environment_proxy::query_by_group_id(
        &state.user_uuid,
        payload.proxy_group_id,
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
    Json(payload): Json<Proxy>,
) -> impl IntoResponse {
    match services::environment_proxy::create(&state.user_uuid, payload).await {
        Ok(data) => AppResponse::<bool>::success(success_message("创建成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("创建失败: {}", r))),
    }
}

pub async fn modify(
    state: Extension<CurrentUser>,
    Json(payload): Json<Proxy>,
) -> impl IntoResponse {
    if let None = payload.id {
        return AppResponse::<bool>::success(success_message("更新失败"), Some(false));
    }

    match services::environment_proxy::update(
        &state.user_uuid,
        payload.id.unwrap_or_default() as u32,
        payload,
    )
    .await
    {
        Ok(data) => AppResponse::<bool>::success(success_message("更新成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn delete(
    state: Extension<CurrentUser>,
    Json(payload): Json<IdPayload>,
) -> impl IntoResponse {
    match services::environment_proxy::delete(&state.user_uuid, payload.id).await {
        Ok(data) => AppResponse::<bool>::success(success_message("删除成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}

pub async fn batch_delete(
    state: Extension<CurrentUser>,
    Json(payload): Json<BatchDeletePayload>,
) -> impl IntoResponse {
    match services::environment_proxy::batch_delete(&state.user_uuid, payload.ids).await {
        Ok(data) => AppResponse::<bool>::success(success_message("删除成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}
