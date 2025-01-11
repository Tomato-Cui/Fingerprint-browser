use crate::handles::environment_group;
use axum::routing::{delete, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environmnet-groups",
        Router::new()
            .route("/query/id", post(environment_group::query_by_id))
            .route("/query", post(environment_group::query))
            .route("/", post(environment_group::create))
            .route("/", put(environment_group::modify))
            .route("/", delete(environment_group::delete)),
    )
}
