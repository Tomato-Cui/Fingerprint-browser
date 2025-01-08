use axum::routing::{post, put};
use axum::Router;

use crate::handles::message;

pub fn build_router() -> Router {
    Router::new().nest(
        "/messages",
        Router::new()
            .route("/user/receive-query", post(message::user_receive_query))
            .route("/team/receive-query", post(message::team_receive_query))
            .route("/team/send", post(message::team_send))
            .route("/user/send", post(message::user_send))
            .route("/reject", post(message::reject))
            .route("/team/allow", put(message::team_allow))
            .route("/user/allow", put(message::user_allow)),
    )
}
