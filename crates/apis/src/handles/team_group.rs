use axum::{response::IntoResponse, Json};

use crate::{
    entities::{team_group::TeamGroupQueryPayload, IdPayload},
    response::AppResponse,
};

use super::{success_message, warn_message};

pub async fn query_by_id(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match services::team_group::query_by_id(payload.id).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_all(Json(payload): Json<TeamGroupQueryPayload>) -> impl IntoResponse {
    match services::team_group::query_by_team_id(payload.team_id).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}
