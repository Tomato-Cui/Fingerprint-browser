use crate::middlewares;
use crate::response::AppResponse;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get, put};
use axum::Extension;
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environment-trash",
        Router::new()
            .route("/:uuid", get(query_id::handle))
            .route("/query", get(query::handle))
            .route("/recover/:id", put(recover::handle))
            .route("/recovers", put(recovers::handle))
            .route("/recover-all", put(recover_all::handle))
            .route("/batch/delete", delete(delete_batch::handle))
            .route("/clean", delete(clean::handle)),
    )
}

mod query_id {
    use super::*;
    use axum::extract::Path;
    use models::environment::Environment;

    pub async fn handle(Path(environment_uuid): Path<String>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environment_trash::query_by_environment_uuid(&environment_uuid).await {
            Ok(data) => AppResponse::<Environment>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Environment>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query {
    use axum::extract::Query;
    use middlewares::CurrentUser;
    use serde_json::Value;

    use super::*;
    use crate::routes::Pagination;

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Query<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environment_trash::query(
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

mod recover {
    use super::*;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Path(environment_uuid): Path<String>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
            Some(format!("恢复失败: {}", v))
        });

        match services::environment_trash::recover(&state.user_uuid, &environment_uuid).await {
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

mod recovers {
    use axum::Json;
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        pub environment_uuids: Vec<String>,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Json(payload): Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
            Some(format!("恢复成功: {}", v))
        });

        match services::environment_trash::recovers(
            &state.user_uuid,
            payload
                .environment_uuids
                .iter()
                .map(|v| v.as_str())
                .collect(),
        )
        .await
        {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod recover_all {
    use super::*;

    pub async fn handle(state: Extension<middlewares::CurrentUser>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
            Some(format!("恢复成功: {}", v))
        });

        match services::environment_trash::recover_all(&state.user_uuid).await {
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

mod delete_batch {
    use axum::Json;
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        pub environment_uuids: Vec<String>,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment_trash::batch_delete_again(
            payload
                .environment_uuids
                .iter()
                .map(|v| v.as_str())
                .collect(),
        )
        .await
        {
            Ok(data) => {
                if data {
                    AppResponse::<bool>::success(success_msg, Some(data))
                } else {
                    AppResponse::<bool>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod clean {
    use super::*;

    pub async fn handle(state: Extension<middlewares::CurrentUser>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("清空成功".to_string()), |v| {
            Some(format!("清空失败: {}", v))
        });

        match services::environment_trash::clean(&state.user_uuid).await {
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
