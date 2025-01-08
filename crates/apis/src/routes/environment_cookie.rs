use crate::handles::environment_cookie;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environment-cookies",
        Router::new()
            .route("/environment", get(environment_cookie::query_by_uuid))
            .route("/environment", post(environment_cookie::create))
            .route("/environment", put(environment_cookie::modify))
            .route("/environment", delete(environment_cookie::delete)),
    )
}
