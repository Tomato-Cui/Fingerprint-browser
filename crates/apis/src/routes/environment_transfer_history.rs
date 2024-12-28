use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::Extension;
use axum::{Json, Router};
use models::environment_transfer_history::EnvironmentTransferHistory;
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environmnet-transfer-historys",
        Router::new()
            .route("/:uuid", get(query_id::handle))
            .route("/query", get(query::handle))
            .route("/batch/create", post(batch_create::handle))
            .route("/delete/:id", delete(delete::handle)),
    )
}

mod query_id {
    use super::*;

    pub async fn handle(Path(uuid): Path<String>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environment_transfer_history::query_by_id(&uuid).await {
            Ok(data) => AppResponse::<EnvironmentTransferHistory>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<EnvironmentTransferHistory>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query {

    use super::*;
    use crate::routes::Pagination;

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Query<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environment_transfer_history::query(
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

mod batch_create {
    use std::collections::HashMap;

    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub environment_uuids: Vec<String>,
        pub user_email: String,
    }
    pub async fn handle(
        state: Extension<CurrentUser>,
        Json(payload): Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        match services::environment_transfer_history::create(
            &state.user_uuid,
            &payload.user_email,
            payload.environment_uuids,
        )
        .await
        {
            Ok(data) => AppResponse::<HashMap<String, bool>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<HashMap<String, bool>>::fail(warn_msg(r.to_string())),
        }
    }
}

mod delete {
    use super::*;

    pub async fn handle(
        state: Extension<CurrentUser>,
        Path(environment_uuid): Path<String>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment_transfer_history::delete(&state.user_uuid, &environment_uuid)
            .await
        {
            Ok(data) => {
                if data {
                    AppResponse::<()>::success(success_msg, Some(()))
                } else {
                    AppResponse::<()>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<()>::fail(warn_msg(r.to_string())),
        }
    }
}
