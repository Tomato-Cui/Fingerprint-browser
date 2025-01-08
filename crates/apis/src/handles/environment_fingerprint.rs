use crate::entities::environment_fingerprint::*;
use crate::entities::Pagination;
use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use models::environment_fingerprint::EnvironmentFingerprint;
use serde_json::Value;

use super::{success_message, warn_message};

pub async fn query_by_id(
    state: Extension<CurrentUser>,
    Json(payload): Json<IdPayload>,
) -> impl IntoResponse {
    match services::environment_fingerprint::query_by_id(&state.user_uuid, payload.id).await {
        Ok(data) => {
            AppResponse::<EnvironmentFingerprint>::success(success_message("查询成功"), Some(data))
        }
        Err(r) => {
            AppResponse::<EnvironmentFingerprint>::fail(warn_message(&format!("查询失败: {}", r)))
        }
    }
}

pub async fn query(
    state: Extension<CurrentUser>,
    Json(payload): Json<Pagination>,
) -> impl IntoResponse {
    match services::environment_fingerprint::query(
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
    Json(payload): Json<EnvironmentFingerprint>,
) -> impl IntoResponse {
    match services::environment_fingerprint::create(&state.user_uuid, &payload).await {
        Ok(data) => AppResponse::<bool>::success(success_message("创建成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("创建失败: {}", r))),
    }
}

pub async fn modify(
    state: Extension<CurrentUser>,
    Json(payload): Json<EnvironmentFingerprint>,
) -> impl IntoResponse {
    if let None = payload.id {
        return AppResponse::<bool>::success(success_message("数据不存在"), Some(false));
    }

    match services::environment_fingerprint::modify(
        &state.user_uuid,
        payload.id.unwrap_or_default() as u32,
        &payload,
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
    match services::environment_fingerprint::delete(&state.user_uuid, payload.id).await {
        Ok(data) => AppResponse::<bool>::success(success_message("删除成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}
