use crate::handles::environment_proxies;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environment-proxies",
        Router::new()
            .route("/id", get(environment_proxies::query_by_id))
            .route("/", get(environment_proxies::query))
            .route("/group", get(environment_proxies::query_by_group))
            .route("/", post(environment_proxies::create))
            .route("/", put(environment_proxies::modify))
            .route("/", delete(environment_proxies::delete))
            .route("/batch", delete(environment_proxies::batch_delete)),
    )
}
