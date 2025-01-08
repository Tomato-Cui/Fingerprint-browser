use crate::handles::environment_fingerprint;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environment-fingerprints",
        Router::new()
            .route("/id", get(environment_fingerprint::query_by_id))
            .route("/", get(environment_fingerprint::query))
            .route("/", post(environment_fingerprint::create))
            .route("/", put(environment_fingerprint::modify))
            .route("/", delete(environment_fingerprint::delete)),
    )
}
