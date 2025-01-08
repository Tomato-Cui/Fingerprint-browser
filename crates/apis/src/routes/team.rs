use crate::handles::team;
use axum::routing::{delete, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/teams",
        Router::new()
            .route("/query", post(team::query))
            .route("/query/id", post(team::query_by_id))
            .route("/query/is-leader", post(team::is_leader))
            .route("/query/search-by-name", post(team::query_search_by_name))
            .route("/query/current", post(team::query_current_team))
            .route("/query/all-user", post(team::query_team_all_user))
            .route(
                "/query/all-blocked-user",
                post(team::query_team_all_blocked_user),
            )
            .route(
                "/query/group/all-user",
                post(team::query_team_group_all_user),
            )
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
