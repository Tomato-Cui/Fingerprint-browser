use crate::response::AppResponse;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environments",
        Router::new()
            .route("/:id", get(query_id::handle))
            .route("/query", get(query::handle))
            .route("/create", post(create::handle))
            .route("/batch", post(batch::handle))
            .route("/batch-import", post(batch::handle))
            .route("/batch-export", post(batch::handle))
            .route("/modify", put(modify::handle))
            .route("/move-to-group", put(move_to_group::handle))
            .route("/batch/move-to-group", put(batch_move_to_group::handle))
            // .route("/grant-permission", put(batch::handle))
            // .route("/batch/grant-permission", put(batch::handle))
            .route("/delete/:id", delete(delete::handle))
            .route("/delete/batch", delete(batch_delete::handle)),
    )
}

mod query_id {
    use super::*;

    pub async fn handle(Path(id): Path<i32>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environment::query_by_id(id).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query {
    use crate::routes::Pagination;

    use super::*;

    pub async fn handle(payload: Query<Pagination>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environment::query(payload.page_num, payload.page_size).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod create {
    use super::*;

    pub async fn handle(payload: Json<models::environment::Environment>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        match services::environment::create(&payload).await {
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

    pub async fn handle(payload: Json<Vec<models::environment::Environment>>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        match services::environment::create_batch(&payload).await {
            Ok(data) => AppResponse::<Vec<Value>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string())),
        }
    }
}

mod modify {
    use super::*;

    pub async fn handle(
        Path(id): Path<u32>,
        mut payload: Json<models::environment::Environment>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });
        payload.id = Some(id as i32);

        match services::environment::modify(&payload).await {
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
        pub environment_id: i32,
        pub group_id: i32,
    }

    pub async fn handle(payload: Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment::move_to_group(payload.environment_id, payload.group_id).await {
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
        pub environment_ids: Vec<i32>,
        pub group_id: i32,
    }

    pub async fn handle(payload: Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment::batch_move_to_group(
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

    pub async fn handle(Path(id): Path<u32>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment::delete(id as i32).await {
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
        pub environment_ids: Vec<i32>,
    }

    pub async fn handle(payload: Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment::batch_delete(payload.environment_ids.clone()).await {
            Ok(data) => AppResponse::<Vec<bool>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<bool>>::fail(warn_msg(r.to_string())),
        }
    }
}
