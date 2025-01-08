use crate::handles::environment_proxy_group;
use axum::routing::{delete, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environmnet-proxy-groups",
        Router::new()
            .route("/query/id", post(environment_proxy_group::query_by_id))
            .route("/query", post(environment_proxy_group::query))
            .route("/", post(environment_proxy_group::create))
            .route("/", put(environment_proxy_group::modify))
            .route("/", delete(environment_proxy_group::delete)),
    )
}
