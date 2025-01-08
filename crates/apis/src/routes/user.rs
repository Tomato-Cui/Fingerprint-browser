use crate::handles::user;
use axum::{
    routing::{get, post},
    Router,
};

pub fn build_router() -> Router {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/login", post(user::handle_login))
            .route("/logout", get(user::handle_logout))
            .route("/register", post(user::handle_register))
            .route("/search-by-email", post(user::handle_search_by_email))
            .route("/reset-password", post(user::handle_reset_password))
            .route("/register/send", post(user::handle_register_send)),
    )
}
