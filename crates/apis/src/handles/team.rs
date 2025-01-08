use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    entities::{team::*, IdPayload},
    middlewares::CurrentUser,
    response::AppResponse,
};

use super::{success_message, warn_message};

pub async fn query_by_id(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match services::team::query_by_id(payload.id).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn is_leader(
    state: Extension<CurrentUser>,
    Path(team_id): Path<u32>,
) -> impl IntoResponse {
    match services::team::is_leader(&state.user_uuid, team_id).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_search_by_name(Path(team_name): Path<String>) -> impl IntoResponse {
    match services::team::query_by_search_name(&team_name).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query(state: Extension<CurrentUser>, payload: Query<Pagination>) -> impl IntoResponse {
    match services::team::query(&state.user_uuid, payload.page_num, payload.page_size).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_current_team(state: Extension<CurrentUser>) -> impl IntoResponse {
    match services::team::query_current_team_info(&state.user_uuid).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_team_all_user(
    state: Extension<CurrentUser>,
    Json(payload): Json<TeamQueryPayload>,
) -> impl IntoResponse {
    match services::team::query_team_all_user(
        &state.user_uuid,
        payload.team_id,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_team_all_blocked_user(
    state: Extension<CurrentUser>,
    payload: Json<TeamQueryPayload>,
) -> impl IntoResponse {
    match services::team::query_team_all_blocked_user(
        &state.user_uuid,
        payload.team_id,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_team_group_all_user(
    state: Extension<CurrentUser>,
    payload: Json<TeamGroupQueryPayload>,
) -> impl IntoResponse {
    match services::team::query_team_group_all_user(
        &state.user_uuid,
        payload.team_id,
        payload.team_group_id,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn blocked(
    state: Extension<CurrentUser>,
    payload: Json<BlockedPayload>,
) -> impl IntoResponse {
    match services::team::blocked(
        &state.user_uuid,
        &payload.current_user_uuid,
        payload.team_id,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("拉黑成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn unblocked(
    state: Extension<CurrentUser>,
    payload: Json<BlockedPayload>,
) -> impl IntoResponse {
    match services::team::un_blocked(
        &state.user_uuid,
        &payload.current_user_uuid,
        payload.team_id,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("取消拉黑成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn create(
    state: Extension<CurrentUser>,
    payload: Json<CreateTeamPayload>,
) -> impl IntoResponse {
    let team = models::team::Team {
        name: payload.name.clone(),
        description: Some(payload.description.clone()),
        ..Default::default()
    };

    match services::team::create(&state.user_uuid, &team).await {
        Ok(_) => AppResponse::success(success_message("创建成功"), Some(())),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn modify(payload: Json<ModifyTeamPayload>) -> impl IntoResponse {
    let id = payload.id as u32;
    let team = models::team::Team {
        id: id as i32,
        name: payload.name.clone(),
        description: Some(payload.description.clone()),
        ..Default::default()
    };

    match services::team::modify(id, &team).await {
        Ok(_) => AppResponse::success(success_message("更新成功"), Some(())),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn switch_team(state: Extension<CurrentUser>, Path(id): Path<u32>) -> impl IntoResponse {
    match services::team::switch_team(&state.user_uuid, id).await {
        Ok(_) => AppResponse::success(success_message("切换成功"), Some(())),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn remove_current_user(
    state: Extension<CurrentUser>,
    payload: Json<BlockedPayload>,
) -> impl IntoResponse {
    match services::team::remove_user(
        &state.user_uuid,
        payload.team_id,
        &payload.current_user_uuid,
    )
    .await
    {
        Ok(_) => AppResponse::success(success_message("移除成功"), Some(true)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn modify_team_user_info(
    state: Extension<CurrentUser>,
    payload: Json<ModifyTeamUserInfoPayload>,
) -> impl IntoResponse {
    match services::team::update_team_user_info(
        &state.user_uuid,
        payload.team_id,
        payload.description.clone(),
        payload.team_group_id,
        &payload.current_user_uuid,
    )
    .await
    {
        Ok(_) => AppResponse::success(success_message("更新成功"), Some(true)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn delete(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match services::team::delete(payload.id).await {
        Ok(_) => AppResponse::success(success_message("删除成功"), Some(())),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}
