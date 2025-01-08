use crate::handles::environment_group;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environmnet-groups",
        Router::new()
            .route("/id", get(environment_group::query_by_id))
            .route("/", get(environment_group::query))
            .route("/", post(environment_group::create))
            .route("/", put(environment_group::modify))
            .route("/", delete(environment_group::delete)),
    )
}

