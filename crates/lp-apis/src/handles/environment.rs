use crate::entities::{environment::*, EnvironmentUUIdPayload};
use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde_json::Value;

use super::{success_message, warn_message};

pub async fn query_by_uuid(
    state: Extension<CurrentUser>,
    Json(payload): Json<EnvironmentUUIdPayload>,
) -> impl IntoResponse {
    match lp_services::environment::query_environment_details(
        &state.user_uuid,
        &payload.environment_uuid,
    )
    .await
    {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn query(
    state: Extension<CurrentUser>,
    Json(payload): Json<Pagination>,
) -> impl IntoResponse {
    let user_id = &state.user_uuid;
    match lp_services::environment::query(user_id, payload.page_num, payload.page_size).await {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn query_by_group(Json(payload): Json<QueryByGroupIdPayload>) -> impl IntoResponse {
    match lp_services::environment::query_by_group_id(
        payload.group_id,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn query_by_extension(
    Json(payload): Json<QueryByExtensionUuidPayload>,
) -> impl IntoResponse {
    match lp_services::extension::query_environmnets_by_extension_uuid(
        &payload.extension_uuid,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(ok) => AppResponse::<Value>::success(success_message("查询成功"), Some(ok)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("查询失败: {}", r))),
    }
}

pub async fn query_by_team(
    state: Extension<CurrentUser>,
    Json(payload): Json<QueryByTeamIdPayload>,
) -> impl IntoResponse {
    match lp_services::environment::query_by_team_id(
        &state.user_uuid,
        payload.team_id,
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
    Json(payload): Json<CreatePayload>,
) -> impl IntoResponse {
    let user_id = &state.user_uuid;
    match lp_services::environment::create(user_id, payload.name).await {
        Ok(ok) => AppResponse::<bool>::success(success_message("创建成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("创建失败: {}", r))),
    }
}

pub async fn detail_create(
    state: Extension<CurrentUser>,
    Json(payload): Json<lp_models::environment::EnvironmentInfo>,
) -> impl IntoResponse {
    match lp_services::environment::create_and_other_info(&state.user_uuid, payload).await {
        Ok(ok) => AppResponse::<bool>::success(success_message("创建成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("创建失败: {}", r))),
    }
}

pub async fn batch_create(
    state: Extension<CurrentUser>,
    Json(payload): Json<BatchCreatePayload>,
) -> impl IntoResponse {
    match lp_services::environment::create_batch(&state.user_uuid, payload.names).await {
        Ok(data) => AppResponse::<Vec<Value>>::success(success_message("创建成功"), Some(data)),
        Err(r) => AppResponse::<Vec<Value>>::fail(warn_message(&format!("创建失败: {}", r))),
    }
}

pub async fn modify_info(
    state: Extension<CurrentUser>,
    Json(payload): Json<lp_models::environment::EnvironmentInfo>,
) -> impl IntoResponse {
    if let None = payload.uuid {
        return AppResponse::<bool>::fail(warn_message(&format!(
            "更新失败: {}",
            anyhow::anyhow!("数据不存在")
        )));
    }

    match lp_services::environment::modify_info(
        &state.user_uuid,
        &payload.uuid.clone().unwrap_or_default(),
        payload,
    )
    .await
    {
        Ok(ok) => AppResponse::<bool>::success(success_message("更新成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn modify_proxy(
    state: Extension<CurrentUser>,
    Json(payload): Json<ModifyProxy>,
) -> impl IntoResponse {
    match lp_services::environment_proxy::update_proxy(
        &state.user_uuid,
        &payload.environment_uuid,
        payload.porxy,
    )
    .await
    {
        Ok(ok) => AppResponse::<bool>::success(success_message("更新成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn modify_basic_info(
    Json(payload): Json<lp_models::environment::Environment>,
) -> impl IntoResponse {
    if let None = payload.uuid {
        return AppResponse::<bool>::fail(warn_message(&format!(
            "更新失败: {}",
            anyhow::anyhow!("数据不存在")
        )));
    }
    match lp_services::environment::modify_basic_info(
        &payload.uuid.unwrap_or_default(),
        &payload.name,
        payload.description,
    )
    .await
    {
        Ok(ok) => AppResponse::<bool>::success(success_message("更新成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn move_to_group(payload: Json<MoveToGroupPayload>) -> impl IntoResponse {
    match lp_services::environment::move_to_group(&payload.environment_uuid, payload.group_id).await {
        Ok(ok) => AppResponse::<bool>::success(success_message("更新成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn batch_move_to_group(payload: Json<BatchMoveToGroupPayload>) -> impl IntoResponse {
    match lp_services::environment::batch_move_to_group(
        payload.environment_ids.clone(),
        payload.group_id,
    )
    .await
    {
        Ok(data) => AppResponse::<Vec<Value>>::success(success_message("更新成功"), Some(data)),
        Err(r) => AppResponse::<Vec<Value>>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn delete(
    state: Extension<CurrentUser>,
    Json(payload): Json<EnvironmentUUIdPayload>,
) -> impl IntoResponse {
    match lp_services::environment::delete(&state.user_uuid, &payload.environment_uuid).await {
        Ok(ok) => AppResponse::<bool>::success(success_message("删除成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}

pub async fn batch_delete(
    state: Extension<CurrentUser>,
    payload: Json<BatchDeletePayload>,
) -> impl IntoResponse {
    match lp_services::environment::batch_delete(&state.user_uuid, payload.environment_uuids.clone())
        .await
    {
        Ok(data) => AppResponse::<Vec<bool>>::success(success_message("删除成功"), Some(data)),
        Err(r) => AppResponse::<Vec<bool>>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}
