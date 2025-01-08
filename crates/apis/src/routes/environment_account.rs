use crate::handles::environment_account;
use axum::routing::{delete, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environment-accounts",
        Router::new()
            .route("/query/id", post(environment_account::query_by_id))
            .route("/query", post(environment_account::query))
            .route(
                "/query/environment/uuid",
                post(environment_account::query_current_by_current_environment),
            )
            .route("/", post(environment_account::create))
            .route("/", put(environment_account::modify))
            .route("/", delete(environment_account::delete))
            .route("/batch", delete(environment_account::batch_delete)),
    )
}
