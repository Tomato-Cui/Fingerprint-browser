use crate::handles::extension;
use axum::routing::{delete, put};
use axum::{routing::post, Router};

pub fn build_router() -> Router {
    Router::new().nest(
        "/extensions",
        Router::new()
            .route("/user/create", post(extension::user_create))
            .route("/team/create", post(extension::team_create))
            .route("/query/user", post(extension::query_by_user))
            .route("/query/team", post(extension::query_by_team))
            .route("/query", post(extension::query))
            .route("/query/environment", post(extension::query_by_environment))
            .route(
                "/environmnet/use",
                post(extension::environment_use_extension),
            )
            .route(
                "/environmnet/remove",
                delete(extension::environment_remove_extension),
            )
            .route("/update", put(extension::update))
            .route(
                "/user/toggle-extension",
                put(extension::user_toggle_extension),
            )
            .route("/delete/uuid", delete(extension::delete_by_uuid))
            .route("/remove/user-uuid", delete(extension::remove_by_user_uuid)),
    )
}
