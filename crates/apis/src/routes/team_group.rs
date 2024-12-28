use crate::response::AppResponse;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use models::team_group::TeamGroup;

pub fn build_router() -> Router {
    Router::new().nest(
        "/team-groups",
        Router::new()
            .route("/:id", get(query_id::handle))
            .route("/query-all", post(query::handle)),
    )
}

mod query_id {

    use super::*;

    pub async fn handle(Path(id): Path<u32>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::team_group::query_by_id(id).await {
            Ok(data) => AppResponse::<TeamGroup>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<TeamGroup>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query {
    use serde_json::Value;

    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub team_id: u32,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::team_group::query_by_team_id(payload.team_id).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}
