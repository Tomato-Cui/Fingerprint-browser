use crate::entities::{environment_trash::*, EnvironmentUUIdPayload, Pagination};
use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde_json::Value;

use super::{success_message, warn_message};

pub async fn query_by_uuid(Json(payload): Json<EnvironmentUUIdPayload>) -> impl IntoResponse {
    match lp_services::environment_trash::query_by_environment_uuid(&payload.environment_uuid).await {
        Ok(data) => AppResponse::<lp_models::environment::Environment>::success(
            success_message("查询成功"),
            Some(data),
        ),
        Err(r) => AppResponse::<lp_models::environment::Environment>::fail(warn_message(&format!(
            "查询失败: {}",
            r
        ))),
    }
}

pub async fn query(
    state: Extension<CurrentUser>,
    Json(payload): Json<Pagination>,
) -> impl IntoResponse {
    match lp_services::environment_trash::query(&state.user_uuid, payload.page_num, payload.page_size)
        .await
    {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn recover(
    state: Extension<CurrentUser>,
    Json(payload): Json<EnvironmentUUIdPayload>,
) -> impl IntoResponse {
    match lp_services::environment_trash::recover(&state.user_uuid, &payload.environment_uuid).await {
        Ok(ok) => AppResponse::<bool>::success(success_message("恢复成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("恢复失败: {}", r))),
    }
}

pub async fn recovers(
    state: Extension<CurrentUser>,
    Json(payload): Json<RecoversPayload>,
) -> impl IntoResponse {
    match lp_services::environment_trash::recovers(
        &state.user_uuid,
        payload
            .environment_uuids
            .iter()
            .map(|v| v.as_str())
            .collect(),
    )
    .await
    {
        Ok(ok) => AppResponse::<bool>::success(success_message("恢复成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("恢复失败: {}", r))),
    }
}

pub async fn recover_all(state: Extension<CurrentUser>) -> impl IntoResponse {
    match lp_services::environment_trash::recover_all(&state.user_uuid).await {
        Ok(ok) => AppResponse::<bool>::success(success_message("恢复成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("恢复失败: {}", r))),
    }
}

pub async fn delete_batch(Json(payload): Json<DeleteBatchPayload>) -> impl IntoResponse {
    match lp_services::environment_trash::batch_delete_again(
        payload
            .environment_uuids
            .iter()
            .map(|v| v.as_str())
            .collect(),
    )
    .await
    {
        Ok(ok) => AppResponse::<bool>::success(success_message("删除成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}

pub async fn clean(state: Extension<CurrentUser>) -> impl IntoResponse {
    match lp_services::environment_trash::clean(&state.user_uuid).await {
        Ok(ok) => AppResponse::<bool>::success(success_message("清空成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("清空失败: {}", r))),
    }
}
