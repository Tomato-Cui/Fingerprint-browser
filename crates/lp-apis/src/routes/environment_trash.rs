use axum::routing::{delete, post, put};
use axum::Router;

use crate::handles::environment_trash;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environment-trash",
        Router::new()
            .route("/query/uuid", post(environment_trash::query_by_uuid))
            .route("/query", post(environment_trash::query))
            .route("/recover", put(environment_trash::recover))
            .route("/recovers", put(environment_trash::recovers))
            .route("/recover-all", put(environment_trash::recover_all))
            .route("/batch", delete(environment_trash::delete_batch))
            .route("/clean", delete(environment_trash::clean)),
    )
}
