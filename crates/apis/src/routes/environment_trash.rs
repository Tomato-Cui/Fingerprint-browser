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
            .route("/:id", get(query_id::handle))
            .route("/query", get(query::handle))
            .route("/recover/:id", put(recover::handle))
            .route("/recovers", put(recovers::handle))
            .route("/recover-all", put(recover_all::handle))
            .route("/delete-again/:id", delete(delete_again::handle))
            .route("/batch/delete", delete(delete_batch::handle))
            .route("/clean", delete(clean::handle)),
    )
}

mod query_id {
    use super::*;
    use axum::extract::Path;
    use middlewares::CurrentUser;
    use models::environment::Environment;

    pub async fn handle(state: Extension<CurrentUser>, Path(id): Path<u32>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        let (id, user_id) = (id, state.id);
        match services::environment::query_by_id(Some(user_id), None, id).await {
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

        let user_id = state.id;
        match services::environment_trash::query(user_id, payload.page_num, payload.page_size).await
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
        Path(id): Path<u32>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
            Some(format!("恢复失败: {}", v))
        });

        match services::environment_trash::recover(state.id, id).await {
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
    use serde_json::Value;

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        pub environment_ids: Vec<u32>,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
            Some(format!("恢复成功: {}", v))
        });

        match services::environment_trash::recovers(state.id, payload.environment_ids.clone()).await
        {
            Ok(data) => AppResponse::<Vec<Value>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string())),
        }
    }
}

mod recover_all {
    use super::*;

    pub async fn handle(state: Extension<middlewares::CurrentUser>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
            Some(format!("恢复成功: {}", v))
        });

        match services::environment_trash::recover_all(state.id).await {
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

mod delete_again {
    use super::*;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Path(id): Path<u32>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment_trash::delete_again(state.id, id).await {
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
        pub ids: Vec<u32>,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Json(payload): Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment_trash::batch_delete_again(state.id, payload.ids).await {
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

        match services::environment_trash::clean(state.id).await {
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
