use crate::handles::environment;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environments",
        Router::new()
            .route("/uuid", get(environment::query_by_uuid))
            .route("/", get(environment::query))
            .route("/group", get(environment::query_by_group))
            .route("/team", get(environment::query_by_team))
            .route("/extension", post(environment::query_by_extension))
            .route("/create", post(environment::create))
            .route("/detail/create", post(environment::detail_create))
            .route("/batch", post(environment::batch_create))
            .route("/batch-import", post(environment::batch_create))
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
