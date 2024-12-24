use axum::{http::StatusCode, middleware, routing::get, Router};

pub mod middlewares;
pub mod response;
pub mod routes;

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "")
}

pub fn build_root_router() -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .merge(routes::user::build_router())
                .merge(routes::environment::build_router())
                .merge(routes::environment_trash::build_router())
                .merge(routes::team::build_router())
                .merge(routes::fingerprint::build_router())
                .merge(routes::group::build_router())
                .merge(routes::proxy::build_router())
                .merge(
                    Router::new()
                        .route("/status", get(|| async { String::from("status: running") })),
                ),
        )
        .route_layer(middleware::from_fn(middlewares::logger))
        .route_layer(middleware::from_fn(middlewares::auth))
        .fallback(not_found)
}
