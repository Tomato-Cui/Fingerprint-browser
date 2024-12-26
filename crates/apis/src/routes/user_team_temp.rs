use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use crate::routes::Pagination;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, post, put};
use axum::Extension;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/messages",
        Router::new()
            .route("/user/receive-query", post(user_receive_query::handle))
            .route("/team/receive-query", post(team_receive_query::handle))
            .route("/team/send", post(team_send::handle))
            .route("/user/send", post(user_send::handle))
            .route("/reject/:id", delete(reject::handle))
            .route("/team/allow", put(team_allow::handle))
            .route("/user/allow", put(user_allow::handle)),
    )
}

mod user_receive_query {
    use serde_json::Value;

    use super::*;

    pub async fn handle(
        state: Extension<CurrentUser>,
        Json(payload): Json<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::user_team_temp::query_user_apply(
            &state.user_uuid,
            payload.page_num,
            payload.page_size,
        )
        .await
        {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod team_receive_query {

    use super::*;

    pub async fn handle(
        state: Extension<CurrentUser>,
        Json(payload): Json<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::user_team_temp::query_team_apply(
            &state.user_uuid,
            payload.page_num,
            payload.page_size,
        )
        .await
        {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod team_send {

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        team_id: u32,
        user_uuid: String,
        description: String,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("发送成功".to_string()), |v| {
            Some(format!("发送失败: {}", v))
        });

        match services::user_team_temp::team_send(
            &payload.user_uuid,
            payload.team_id,
            &payload.description,
        )
        .await
        {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod user_send {

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        team_id: u32,
        description: String,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        Json(payload): Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("发送成功".to_string()), |v| {
            Some(format!("发送失败: {}", v))
        });

        match services::user_team_temp::user_send(
            &state.user_uuid,
            payload.team_id,
            &payload.description,
        )
        .await
        {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod reject {
    use super::*;
    pub async fn handle(Path(id): Path<u32>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("拒绝成功".to_string()), |v| {
            Some(format!("拒绝失败: {}", v))
        });

        match services::user_team_temp::delete(id).await {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod team_allow {

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        id: u32,
        user_uuid: String,
        team_id: u32,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("通过成功".to_string()), |v| {
            Some(format!("通过失败: {}", v))
        });

        match services::user_team_temp::allow(payload.id, &payload.user_uuid, payload.team_id).await
        {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod user_allow {

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        id: u32,
        team_id: u32,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        Json(payload): Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("通过成功".to_string()), |v| {
            Some(format!("通过失败: {}", v))
        });

        match services::user_team_temp::allow(payload.id, &state.user_uuid, payload.team_id).await {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}
