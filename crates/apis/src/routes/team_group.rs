use crate::handles::team_group;
use axum::{
    routing::{get, post},
    Router,
};

pub fn build_router() -> Router {
    Router::new().nest(
        "/team-groups",
        Router::new()
            .route("/id", get(team_group::query_by_id))
            .route("/", post(team_group::query_all)),
    )
}
