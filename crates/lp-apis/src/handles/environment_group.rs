use super::{success_message, warn_message};
use crate::entities::{environment_group::*, Pagination};
use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use lp_models::environment_group::EnvironmentGroup;
use serde_json::Value;

pub async fn query_by_id(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match lp_services::environment_group::query_by_id(payload.id).await {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn query(
    state: Extension<CurrentUser>,
    Json(payload): Json<Pagination>,
) -> impl IntoResponse {
    match lp_services::environment_group::query(&state.user_uuid, payload.page_num, payload.page_size)
        .await
    {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn create(
    state: Extension<CurrentUser>,
    Json(payload): Json<CreatePayload>,
) -> impl IntoResponse {
    let group = EnvironmentGroup {
        user_uuid: state.user_uuid.to_string(),
        name: payload.name.clone(),
        description: payload.description.clone(),
        ..Default::default()
    };

    match lp_services::environment_group::create(&group).await {
        Ok(data) => AppResponse::<bool>::success(success_message("创建成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("创建失败: {}", r))),
    }
}

pub async fn modify(Json(payload): Json<ModifyPayload>) -> impl IntoResponse {
    let group = EnvironmentGroup {
        name: payload.name.clone(),
        description: payload.description.clone(),
        ..Default::default()
    };

    match lp_services::environment_group::modify(payload.id, &group).await {
        Ok(data) => AppResponse::<bool>::success(success_message("更新成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn delete(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match lp_services::environment_group::delete(payload.id).await {
        Ok(data) => AppResponse::<bool>::success(success_message("删除成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}
