use crate::handles::environment_account;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environment-accounts",
        Router::new()
            .route("/id", get(environment_account::query_by_id))
            .route("/", get(environment_account::query))
            .route("/user", get(environment_account::query_current_by_user))
            .route("/", post(environment_account::create))
            .route("/", put(environment_account::modify))
            .route("/", delete(environment_account::delete))
            .route("/batch", delete(environment_account::batch_delete)),
    )
}
