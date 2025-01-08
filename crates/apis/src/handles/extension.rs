use super::{success_message, warn_message};
use crate::{
    entities::{extension::*, Pagination},
    middlewares::CurrentUser,
    response::AppResponse,
};
use axum::{response::IntoResponse, Extension, Json};

pub async fn user_create(
    state: Extension<CurrentUser>,
    Json(payload): Json<ExtensionUUidPayload>,
) -> impl IntoResponse {
    match services::extension::user_insert(&state.user_uuid, &payload.extension_uuid).await {
        Ok(ok) => AppResponse::success(success_message("创建成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn team_create(Json(payload): Json<TeamCreatePayload>) -> impl IntoResponse {
    match services::extension::team_insert(&payload.team_id, &payload.extension_uuid).await {
        Ok(ok) => AppResponse::success(success_message("创建成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_by_team(Json(payload): Json<QueryByTeamPayload>) -> impl IntoResponse {
    match services::extension::query_by_team_id(
        payload.team_id,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(ok) => AppResponse::success(success_message("查询成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_by_user(
    state: Extension<CurrentUser>,
    Json(payload): Json<Pagination>,
) -> impl IntoResponse {
    match services::extension::query_by_user_uuid(
        &state.user_uuid,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(ok) => AppResponse::success(success_message("查询成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_by_environment(
    Json(payload): Json<QueryByEnvironmentUuidPayload>,
) -> impl IntoResponse {
    match services::extension::query_by_environmnet_uuid(
        &payload.environment_uuid,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(ok) => AppResponse::success(success_message("查询成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query(Json(payload): Json<Pagination>) -> impl IntoResponse {
    match services::extension::query(payload.page_num, payload.page_size).await {
        Ok(ok) => AppResponse::success(success_message("查询成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn environment_use_extension(
    Json(payload): Json<EnvironmentUseExtensionPayload>,
) -> impl IntoResponse {
    match services::extension::environmnet_use_extension(
        &payload.extension_uuid,
        payload.environment_uuids,
    )
    .await
    {
        Ok(ok) => AppResponse::success(success_message("更新成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn environment_remove_extension(
    Json(payload): Json<EnvironmentRemoveExtensionPayload>,
) -> impl IntoResponse {
    match services::extension::environmnet_remove_extension(
        &payload.extension_uuid,
        &payload.environment_uuid,
    )
    .await
    {
        Ok(ok) => AppResponse::success(success_message("移除成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn update(Json(payload): Json<UpdatePayload>) -> impl IntoResponse {
    match services::extension::update(&payload.extension_uuid, payload.extension).await {
        Ok(ok) => AppResponse::success(success_message("更新成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn user_toggle_extension(
    state: Extension<CurrentUser>,
    Json(payload): Json<UserToggleExtensionPayload>,
) -> impl IntoResponse {
    match services::extension::user_toggle_extension(
        &state.user_uuid,
        &payload.extension_uuid,
        payload.open,
    )
    .await
    {
        Ok(ok) => AppResponse::success(success_message("更新成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn delete_by_uuid(Json(payload): Json<ExtensionUUidPayload>) -> impl IntoResponse {
    match services::extension::delete(&payload.extension_uuid).await {
        Ok(ok) => AppResponse::success(success_message("删除成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn remove_by_user_uuid(
    state: Extension<CurrentUser>,
    Json(payload): Json<ExtensionUUidPayload>,
) -> impl IntoResponse {
    match services::extension::remove_by_user_uuid(&state.user_uuid, &payload.extension_uuid).await
    {
        Ok(ok) => AppResponse::success(success_message("移除成功"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}
