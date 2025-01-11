use crate::handles::browser;
use axum::routing::{get, post};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/browsers",
        Router::new()
            .route("/start", post(browser::start))
            .route("/starts", post(browser::starts))
            .route("/stops", post(browser::stops))
            .route("/stop-all", post(browser::stop_all))
            .route("/status", get(browser::status)),
    )
}
