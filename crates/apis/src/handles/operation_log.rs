use super::{success_message, warn_message};
use crate::{
    entities::{environment::QueryByTeamIdPayload, Pagination},
    middlewares::CurrentUser,
    response::AppResponse,
};
use axum::{response::IntoResponse, Extension, Json};

pub async fn query(
    state: Extension<CurrentUser>,
    Json(payload): Json<Pagination>,
) -> impl IntoResponse {
    match services::operation_log::query_by_user_uuid(
        &state.user_uuid,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(ok) => AppResponse::success(success_message("查询失败"), Some(ok)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_by_team(Json(payload): Json<QueryByTeamIdPayload>) -> impl IntoResponse {
    match services::operation_log::query_by_team_id(
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
