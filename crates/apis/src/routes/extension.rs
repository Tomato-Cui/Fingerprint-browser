use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use crate::routes::Pagination;
use axum::extract::Path;
use axum::routing::{delete, put};
use axum::Extension;
use axum::{response::IntoResponse, routing::post, Json, Router};
use models::extension;
use serde::Deserialize;
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/extensions",
        Router::new()
            .route("/user-create", post(user_create::handle))
            .route("/team-create", post(team_create::handle))
            .route("/query-by-user", post(query_by_team::handle))
            .route("/query-by-user", post(query_by_user::handle))
            .route(
                "/query-by-environment/:uuid",
                post(query_by_environmnet::handle),
            )
            .route("/query", post(query::handle))
            .route(
                "/environmnet-use-extension",
                post(environmnet_use_extension::handle),
            )
            .route(
                "/environmnet-remove-extension",
                delete(environmnet_remove_extension::handle),
            )
            .route("/update", put(update::handle))
            .route("/user-toggle-extension", put(user_toggle_extension::handle))
            .route("/delete-by-uuid", delete(delete_by_uuid::handle))
            .route("/remove-by-user-uuid", delete(remove_by_user_uuid::handle)),
    )
}

mod user_create {

    use super::*;

    pub async fn handle(
        state: Extension<CurrentUser>,
        Path(extension_uuid): Path<String>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        match services::extension::user_insert(&state.user_uuid, &extension_uuid).await {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod team_create {
    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        team_id: String,
        extension_uuid: String,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        match services::extension::team_insert(&payload.team_id, &payload.extension_uuid).await {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query_by_team {
    use serde_json::Value;

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        team_id: u32,
        page_num: u32,
        page_size: u32,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        match services::extension::query_by_team_id(
            payload.team_id,
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

mod query_by_user {
    use serde_json::Value;

    use super::*;

    pub async fn handle(
        state: Extension<CurrentUser>,
        Json(payload): Json<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::extension::query_by_user_uuid(
            &state.user_uuid,
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

mod query_by_environmnet {

    use super::*;

    pub async fn handle(
        Path(environmnet_uuid): Path<String>,
        Json(payload): Json<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::extension::query_by_environmnet_uuid(
            &environmnet_uuid,
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

mod query {
    use super::*;

    pub async fn handle(Json(payload): Json<Pagination>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::extension::query(payload.page_num, payload.page_size).await {
            Ok(ok) => AppResponse::<Value>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod environmnet_use_extension {
    use std::collections::HashMap;

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        extension_uuid: String,
        environment_uuids: Vec<String>,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::extension::environmnet_use_extension(
            &payload.extension_uuid,
            payload.environment_uuids,
        )
        .await
        {
            Ok(ok) => AppResponse::<HashMap<String, bool>>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<HashMap<String, bool>>::fail(warn_msg(r.to_string())),
        }
    }
}

mod environmnet_remove_extension {
    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        extension_uuid: String,
        environment_uuid: String,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("移除成功".to_string()), |v| {
            Some(format!("移除失败: {}", v))
        });

        match services::extension::environmnet_remove_extension(
            &payload.extension_uuid,
            &payload.environment_uuid,
        )
        .await
        {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod update {
    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        extension_uuid: String,
        extension: extension::Extension,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::extension::update(&payload.extension_uuid, payload.extension).await {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod user_toggle_extension {

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        extension_uuid: String,
        open: bool,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        Path(payload): Path<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        match services::extension::user_toggle_extension(
            &state.user_uuid,
            &payload.extension_uuid,
            payload.open,
        )
        .await
        {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod delete_by_uuid {
    use super::*;

    pub async fn handle(Path(extension_uuid): Path<String>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::extension::delete(&extension_uuid).await {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod remove_by_user_uuid {
    use super::*;

    pub async fn handle(
        state: Extension<CurrentUser>,
        Path(extension_uuid): Path<String>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("移除成功".to_string()), |v| {
            Some(format!("移除失败: {}", v))
        });

        match services::extension::remove_by_user_uuid(&state.user_uuid, &extension_uuid).await {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}
