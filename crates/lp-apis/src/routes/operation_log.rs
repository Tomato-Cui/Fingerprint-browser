use crate::handles::operation_log;
use axum::routing::post;
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/operation-log",
        Router::new()
            .route("/query", post(operation_log::query))
            .route("/query/team", post(operation_log::query_by_team)),
    )
}
