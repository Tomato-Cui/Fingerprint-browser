use crate::middlewares::CurrentUser;
use crate::response::AppResponse;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::Extension;
use axum::{Json, Router};
use models::team::Team;
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/teams",
        Router::new()
            .route("/:id", get(query_id::handle))
            .route("/query", get(query::handle))
            .route("/query/current-team", get(query_current_team::handle))
            .route("/query/team-all-user", post(query_team_all_user::handle))
            .route(
                "/query/team-all-blocked-user",
                post(query_team_all_blocked_user::handle),
            )
            .route(
                "/query/team-group-all-user",
                post(query_team_group_all_user::handle),
            )
            .route("/blocked", put(blocked::handle))
            .route("/un-blocked", put(unblocked::handle))
            .route("/create", post(create::handle))
            .route("/modify/:id", put(modify::handle))
            .route("/remove-current-user", put(remove_current_user::handle))
            .route("/modify/team-user-info", put(modify_team_user_info::handle))
            .route("/delete/:id", delete(delete::handle)),
    )
}

mod query_id {

    use super::*;

    pub async fn handle(Path(id): Path<u32>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::team::query_by_id(id).await {
            Ok(data) => AppResponse::<Team>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Team>::fail(warn_msg(r.to_string())),
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

        match services::team::query(&state.user_uuid, payload.page_num, payload.page_size).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}
mod query_current_team {
    use super::*;

    pub async fn handle(state: Extension<CurrentUser>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::team::query_current_team_info(&state.user_uuid).await {
            Ok(data) => AppResponse::<Team>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Team>::fail(warn_msg(r.to_string())),
        }
    }
}

mod query_team_all_user {

    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        team_id: u32,
        page_num: u32,
        page_size: u32,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Query<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::team::query_team_all_user(
            &state.user_uuid,
            payload.team_id,
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

mod query_team_all_blocked_user {

    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        team_id: u32,
        page_num: u32,
        page_size: u32,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::team::query_team_all_blocked_user(
            &state.user_uuid,
            payload.team_id,
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

mod query_team_group_all_user {

    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        team_id: u32,
        team_group_id: u32,
        page_num: u32,
        page_size: u32,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::team::query_team_group_all_user(
            &state.user_uuid,
            payload.team_id,
            payload.team_group_id,
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

mod blocked {

    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        current_user_uuid: String,
        team_id: u32,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("拉黑成功".to_string()), |v| {
            Some(format!("拉黑失败: {}", v))
        });

        match services::team::blocked(
            &state.user_uuid,
            &payload.current_user_uuid,
            payload.team_id,
        )
        .await
        {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod unblocked {

    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        current_user_uuid: String,
        team_id: u32,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("取消拉黑成功".to_string()), |v| {
            Some(format!("取消拉黑失败: {}", v))
        });

        match services::team::un_blocked(
            &state.user_uuid,
            &payload.current_user_uuid,
            payload.team_id,
        )
        .await
        {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        }
    }
}

mod create {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub name: String,
        pub description: String,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });
        let team = models::team::Team {
            name: payload.name.clone(),
            description: Some(payload.description.clone()),
            ..Default::default()
        };

        match services::team::create(&state.user_uuid, &team).await {
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
        pub name: String,
        pub description: String,
    }

    pub async fn handle(Path(id): Path<u32>, payload: Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });
        let team = models::team::Team {
            id: id as i32,
            name: payload.name.clone(),
            description: Some(payload.description.clone()),
            ..Default::default()
        };

        match services::team::modify(id, &team).await {
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

mod remove_current_user {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        team_id: u32,
        current_user_uuid: String,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("移除成功".to_string()), |v| {
            Some(format!("移除失败: {}", v))
        });

        match services::team::remove_user(
            &state.user_uuid,
            payload.team_id,
            &payload.current_user_uuid,
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

mod modify_team_user_info {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        team_id: u32,
        description: Option<String>,
        team_group_id: u32,
        current_user_uuid: String,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("移除成功".to_string()), |v| {
            Some(format!("移除失败: {}", v))
        });

        match services::team::update_team_user_info(
            &state.user_uuid,
            payload.team_id,
            payload.description.clone(),
            payload.team_group_id,
            &payload.current_user_uuid,
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

mod delete {
    use super::*;

    pub async fn handle(Path(id): Path<u32>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::team::delete(id).await {
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
