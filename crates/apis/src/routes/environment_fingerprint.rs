use crate::handles::environment_fingerprint;
use axum::routing::{delete, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environment-fingerprints",
        Router::new()
            .route("/query/id", post(environment_fingerprint::query_by_id))
            .route("/query", post(environment_fingerprint::query))
            .route("/", post(environment_fingerprint::create))
            .route("/", put(environment_fingerprint::modify))
            .route("/", delete(environment_fingerprint::delete)),
    )
}
