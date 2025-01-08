use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde_json::Value;

use crate::entities::environment_account::*;
use crate::entities::Pagination;
use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use models::environment_account::EnvironmentAccount;

use super::{success_message, warn_message};

pub async fn query_by_id(Json(payload): Json<QueryByIdPayload>) -> impl IntoResponse {
    match services::environment_account::query_by_id(payload.id).await {
        Ok(data) => {
            AppResponse::<EnvironmentAccount>::success(success_message("查询成功"), Some(data))
        }
        Err(r) => {
            AppResponse::<EnvironmentAccount>::fail(warn_message(&format!("查询失败: {}", r)))
        }
    }
}

pub async fn query(
    state: Extension<CurrentUser>,
    Json(payload): Json<Pagination>,
) -> impl IntoResponse {
    match services::environment_account::query(
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

pub async fn query_current_by_current_environment(
    Json(payload): Json<QueryByEnvironmentUUidPayload>,
) -> impl IntoResponse {
    match services::environment_account::account_query_current_environment(
        &payload.environment_uuid,
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
    Json(payload): Json<EnvironmentAccount>,
) -> impl IntoResponse {
    match services::environment_account::create(&state.user_uuid, payload).await {
        Ok(data) => AppResponse::<bool>::success(success_message("创建成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("创建失败: {}", r))),
    }
}

pub async fn modify(Json(payload): Json<EnvironmentAccount>) -> impl IntoResponse {
    if let None = payload.id {
        return AppResponse::<bool>::success(
            success_message(&format!("更新失败: 数据不存在")),
            Some(false),
        );
    }

    match services::environment_account::modify(&payload).await {
        Ok(data) => AppResponse::<bool>::success(success_message("更新成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn delete(Json(payload): Json<DeleteByIdPayload>) -> impl IntoResponse {
    match services::environment_account::delete(payload.id).await {
        Ok(data) => AppResponse::<bool>::success(success_message("删除成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}

pub async fn batch_delete(Json(payload): Json<BatchDeletePayload>) -> impl IntoResponse {
    match services::environment_account::batch_delete(payload.ids).await {
        Ok(data) => AppResponse::<bool>::success(success_message("删除成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}
