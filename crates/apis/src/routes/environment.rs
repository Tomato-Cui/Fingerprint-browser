use crate::middlewares;
use crate::response::AppResponse;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::Extension;
use axum::{Json, Router};
use middlewares::CurrentUser;
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environments",
        Router::new()
            .route("/:id", get(query_id::handle))
            .route("/query", get(query::handle))
            .route("/query-by-group/:id", get(query_by_group::handle))
            .route("/create", post(create::handle))
            .route("/batch", post(batch::handle))
            .route("/batch-import", post(batch::handle))
            .route("/batch-export", post(query::handle))
            .route("/modify/:id", put(modify::handle))
            .route("/move-to-group", put(move_to_group::handle))
            .route("/batch/move-to-group", put(batch_move_to_group::handle))
            // .route("/grant-permission", put(batch::handle))
            // .route("/batch/grant-permission", put(batch::handle))
            .route("/delete/:id", delete(delete::handle))
            .route("/delete/batch", delete(batch_delete::handle))
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
    use middlewares::CurrentUser;

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
        match services::environment::query(user_id, payload.page_num, payload.page_size).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query_by_group {

    use super::*;
    use crate::routes::Pagination;

    pub async fn handle(
        state: Extension<CurrentUser>,
        Path(id): Path<u32>,
        payload: Query<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        let (user_id, group_id) = (state.id, id);
        match services::environment::query_by_group_id(
            user_id,
            group_id,
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

mod create {
    use super::*;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        payload: Json<models::environment::Environment>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });
        let user_id = state.id;

        match services::environment::create(user_id, &payload).await {
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

mod batch {
    use super::*;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        mut payload: Json<Vec<models::environment::Environment>>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        payload
            .iter_mut()
            .for_each(|v| v.owner_id = Some(state.id as i32));
        let user_id = state.id;

        match services::environment::create_batch(user_id, &payload).await {
            Ok(data) => AppResponse::<Vec<Value>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string())),
        }
    }
}

mod modify {
    use super::*;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Path(id): Path<u32>,
        mut payload: Json<models::environment::Environment>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        let user_id = state.id;
        payload.id = Some(id as i32);
        match services::environment::modify(user_id, &payload).await {
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

mod move_to_group {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub environment_id: u32,
        pub group_id: u32,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment::move_to_group(
            state.id,
            payload.environment_id,
            payload.group_id,
        )
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

mod batch_move_to_group {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub environment_ids: Vec<u32>,
        pub group_id: u32,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment::batch_move_to_group(
            state.id,
            payload.environment_ids.clone(),
            payload.group_id,
        )
        .await
        {
            Ok(data) => AppResponse::<Vec<Value>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string())),
        }
    }
}

mod delete {
    use super::*;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Path(id): Path<u32>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment::delete(state.id, id).await {
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

mod batch_delete {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub environment_ids: Vec<u32>,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment::batch_delete(state.id, payload.environment_ids.clone()).await {
            Ok(data) => AppResponse::<Vec<bool>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<bool>>::fail(warn_msg(r.to_string())),
        }
    }
}
