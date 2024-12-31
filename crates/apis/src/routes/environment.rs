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
            .route("/:uuid", get(query_id::handle))
            .route("/query", get(query::handle))
            .route("/query-by-group/:id", get(query_by_group::handle))
            .route("/query-by-team/:id", get(query_by_team::handle))
            .route(
                "/query-by-extension/:uuid",
                post(query_by_extension::handle),
            )
            .route("/create", post(create::handle))
            .route("/detail/create", post(detail_create::handle))
            .route("/batch", post(batch::handle))
            .route("/batch-import", post(batch::handle))
            .route("/batch-export", post(query::handle))
            .route("/modify-basic-info/:uuid", put(modify_basic_info::handle))
            .route("/modify-info/:uuid", put(modify_info::handle))
            .route("/modify-proxy/:uuid", put(modify_proxy::handle))
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
    use axum::extract::Path;

    pub async fn handle(
        state: Extension<CurrentUser>,
        Path(uuid): Path<String>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environment::query_environment_details(&state.user_uuid, &uuid).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
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
        let user_id = &state.user_uuid;
        match services::environment::query(user_id, payload.page_num, payload.page_size).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query_by_group {

    use super::*;
    use crate::routes::Pagination;

    pub async fn handle(Path(id): Path<u32>, payload: Query<Pagination>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environment::query_by_group_id(id, payload.page_num, payload.page_size)
            .await
        {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query_by_extension {
    use serde_json::Value;

    use crate::routes::Pagination;

    use super::*;

    pub async fn handle(
        Path(extension_uuid): Path<String>,
        Json(payload): Json<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::extension::query_environmnets_by_extension_uuid(
            &extension_uuid,
            payload.page_num,
            payload.page_size,
        )
        .await
        {
            Ok(ok) => AppResponse::<Value>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query_by_team {

    use super::*;
    use crate::routes::Pagination;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Path(id): Path<u32>,
        payload: Query<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environment::query_by_team_id(
            &state.user_uuid,
            id,
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

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub name: String,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Json(payload): Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });
        let user_id = &state.user_uuid;

        match services::environment::create(user_id, payload.name).await {
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

mod detail_create {
    use super::*;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Json(payload): Json<models::environment::EnvironmentInfo>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        match services::environment::create_and_other_info(&state.user_uuid, payload).await {
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

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub names: Vec<String>,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Json(payload): Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        match services::environment::create_batch(&state.user_uuid, payload.names).await {
            Ok(data) => AppResponse::<Vec<Value>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string())),
        }
    }
}

mod modify_info {
    use super::*;

    pub async fn handle(
        Path(uuid): Path<String>,
        Json(payload): Json<models::environment::EnvironmentInfo>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment::modify_info(
            &uuid,
            &payload.uuid.clone().unwrap_or_default(),
            payload,
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

mod modify_proxy {
    use super::*;

    pub async fn handle(
        Path(uuid): Path<String>,
        Json(payload): Json<models::environment_proxies::Proxy>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment_proxy::update_proxy(&uuid, payload).await {
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

mod modify_basic_info {
    use super::*;

    pub async fn handle(
        Path(uuid): Path<String>,
        Json(payload): Json<models::environment::Environment>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment::modify_basic_info(&uuid, &payload.name, payload.description)
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

mod move_to_group {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub environment_uuid: String,
        pub group_id: u32,
    }

    pub async fn handle(payload: Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment::move_to_group(&payload.environment_uuid, payload.group_id)
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
        pub environment_ids: Vec<String>,
        pub group_id: u32,
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

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Path(environmnet_uuid): Path<String>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment::delete(&state.user_uuid, &environmnet_uuid).await {
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
        pub environment_uuids: Vec<String>,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environment::batch_delete(
            &state.user_uuid,
            payload.environment_uuids.clone(),
        )
        .await
        {
            Ok(data) => AppResponse::<Vec<bool>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<bool>>::fail(warn_msg(r.to_string())),
        }
    }
}
