use crate::handles::environment_proxy_group;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environmnet-proxy-groups",
        Router::new()
            .route("/id", get(environment_proxy_group::query_by_id))
            .route("/", get(environment_proxy_group::query))
            .route("/", post(environment_proxy_group::create))
            .route("/", put(environment_proxy_group::modify))
            .route("/", delete(environment_proxy_group::delete)),
    )
}
