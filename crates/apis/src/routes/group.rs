use crate::middlewares;
use crate::response::AppResponse;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::Extension;
use axum::{Json, Router};
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/groups",
        Router::new()
            .route("/:id", get(query_id::handle))
            .route("/query", get(query::handle))
            .route("/create", post(create::handle))
            .route("/modify", put(modify::handle))
            .route("/grant-user", put(grant_user::handle))
            .route("/delete/:id", delete(delete::handle)),
    )
}

mod query_id {
    use super::*;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        Path(id): Path<u32>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        let user_id = state.id;
        match services::group::query_by_id(id, user_id).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query {
    use super::*;
    use crate::routes::Pagination;

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        payload: Query<Pagination>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        let user_id = state.id;
        match services::group::query(user_id, payload.page_num, payload.page_size).await {
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
        pub description: Option<String>,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        let group = models::group::Group {
            name: payload.name.clone(),
            description: payload.description.clone(),
            owner_id: state.id as i32,
            ..Default::default()
        };

        match services::group::create(state.id, &group).await {
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

mod modify {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub id: u32,
        pub name: String,
        pub description: Option<String>,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });
        let group = models::group::Group {
            id: payload.id as i32,
            name: payload.name.clone(),
            description: payload.description.clone(),
            ..Default::default()
        };

        match services::group::modify(state.id, &group).await {
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

mod grant_user {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub id: u32,
    }

    pub async fn handle(
        state: Extension<middlewares::CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::group::grant_user(state.id, payload.id).await {
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

mod delete {
    use middlewares::CurrentUser;

    use super::*;

    pub async fn handle(state: Extension<CurrentUser>, Path(id): Path<u32>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::group::delete(state.id, id).await {
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

