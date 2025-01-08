use super::{success_message, warn_message};
use crate::entities::IdPayload;
use crate::{entities::message::*, middlewares::CurrentUser, response::AppResponse};
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use serde_json::Value;

pub async fn user_receive_query(
    state: Extension<CurrentUser>,
    Json(payload): Json<Pagination>,
) -> impl IntoResponse {
    match services::user_team_temp::query_user_apply(
        &state.user_uuid,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(r)),
    }
}

pub async fn team_receive_query(Json(payload): Json<TeamReceiveQueryPayload>) -> impl IntoResponse {
    match services::user_team_temp::query_team_apply(
        payload.team_id,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::<Value>::success(success_message("查询成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(r)),
    }
}

pub async fn team_send(Json(payload): Json<TeamSendPayload>) -> impl IntoResponse {
    match services::user_team_temp::team_send(
        &payload.user_uuid,
        payload.team_id,
        &payload.description,
    )
    .await
    {
        Ok(data) => AppResponse::<bool>::success(success_message("发送成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(r)),
    }
}

pub async fn user_send(
    state: Extension<CurrentUser>,
    Json(payload): Json<UserSendPayload>,
) -> impl IntoResponse {
    match services::user_team_temp::user_send(
        &state.user_uuid,
        &payload.team_name,
        &payload.description,
    )
    .await
    {
        Ok(data) => AppResponse::<bool>::success(success_message("发送成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(r)),
    }
}

pub async fn reject(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match services::user_team_temp::delete(payload.id).await {
        Ok(data) => AppResponse::<bool>::success(success_message("拒绝成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(r)),
    }
}

pub async fn team_allow(Json(payload): Json<TeamAllowPayload>) -> impl IntoResponse {
    match services::user_team_temp::allow(payload.id, &payload.user_uuid, payload.team_id).await {
        Ok(data) => AppResponse::<bool>::success(success_message("通过成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(r)),
    }
}

pub async fn user_allow(
    state: Extension<CurrentUser>,
    Json(payload): Json<UserAllowPayload>,
) -> impl IntoResponse {
    match services::user_team_temp::allow(payload.id, &state.user_uuid, payload.team_id).await {
        Ok(data) => AppResponse::<bool>::success(success_message("通过成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(r)),
    }
}
