use crate::handles::environment_transfer_history;
use axum::routing::{delete, post};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environmnet-transfer-historys",
        Router::new()
            .route("/query/id", post(environment_transfer_history::query_by_uuid))
            .route("/query", post(environment_transfer_history::query))
            .route("/batch", post(environment_transfer_history::batch_create))
            .route("/", delete(environment_transfer_history::delete_by_uuid)),
    )
}
