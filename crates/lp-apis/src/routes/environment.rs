use crate::handles::environment;
use axum::routing::{delete, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environments",
        Router::new()
            .route("/query/uuid", post(environment::query_by_uuid))
            .route("/query", post(environment::query))
            .route("/query/group", post(environment::query_by_group))
            .route("/query/team", post(environment::query_by_team))
            .route("/query/extension", post(environment::query_by_extension))
            .route("/batch-export", post(environment::query))
            .route("/basic", put(environment::modify_basic_info))
            .route("/", put(environment::modify_info))
            .route("/proxy", put(environment::modify_proxy))
            .route("/move-to-group", put(environment::move_to_group))
            .route(
                "/batch/move-to-group",
                put(environment::batch_move_to_group),
            )
            .route("/", delete(environment::delete))
            .route("/batch", delete(environment::batch_delete)),
    )
}
