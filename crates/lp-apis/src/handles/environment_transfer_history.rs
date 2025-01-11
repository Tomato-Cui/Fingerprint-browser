use crate::entities::environment_transfer_history::BatchCreatePayload;
use crate::entities::{EnvironmentUUIdPayload, Pagination};
use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use lp_models::environment_transfer_history::EnvironmentTransferHistory;
use serde_json::Value;
use std::collections::HashMap;

use super::{success_message, warn_message};

pub async fn query_by_uuid(Json(payload): Json<EnvironmentUUIdPayload>) -> impl IntoResponse {
    match lp_services::environment_transfer_history::query_by_id(&payload.environment_uuid).await {
        Ok(data) => AppResponse::<EnvironmentTransferHistory>::success(
            success_message("查询成功"),
            Some(data),
        ),
        Err(r) => AppResponse::<EnvironmentTransferHistory>::fail(warn_message(&format!(
            "查询失败: {}",
            r
        ))),
    }
}

pub async fn query(
    state: Extension<CurrentUser>,
    Json(payload): Json<Pagination>,
) -> impl IntoResponse {
    match lp_services::environment_transfer_history::query(
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

pub async fn batch_create(
    state: Extension<CurrentUser>,
    Json(payload): Json<BatchCreatePayload>,
) -> impl IntoResponse {
    match lp_services::environment_transfer_history::create(
        &state.user_uuid,
        &payload.user_email,
        payload.environment_uuids,
    )
    .await
    {
        Ok(data) => {
            AppResponse::<HashMap<String, bool>>::success(success_message("创建成功"), Some(data))
        }
        Err(r) => {
            AppResponse::<HashMap<String, bool>>::fail(warn_message(&format!("创建失败: {}", r)))
        }
    }
}

pub async fn delete_by_uuid(
    state: Extension<CurrentUser>,
    Json(payload): Json<EnvironmentUUIdPayload>,
) -> impl IntoResponse {
    match lp_services::environment_transfer_history::delete(
        &state.user_uuid,
        &payload.environment_uuid,
    )
    .await
    {
        Ok(ok) => AppResponse::<bool>::success(success_message("删除成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}
