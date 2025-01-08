use crate::handles::team;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/teams",
        Router::new()
            .route("/", get(team::query))
            .route("/id", get(team::query_by_id))
            .route("/is-leader", get(team::is_leader))
            .route("/search-by-name", get(team::query_search_by_name))
            .route("/current", get(team::query_current_team))
            .route("/all-user", post(team::query_team_all_user))
            .route("/all-blocked-user", post(team::query_team_all_blocked_user))
            .route("/group/all-user", post(team::query_team_group_all_user))
            .route("/blocked", put(team::blocked))
            .route("/un-blocked", put(team::unblocked))
            .route("/create", post(team::create))
            .route("/", put(team::modify))
            .route("/switch", put(team::switch_team))
            .route("/remove-user", put(team::remove_current_user))
            .route("/user-info", put(team::modify_team_user_info))
            .route("/", delete(team::delete)),
    )
}
