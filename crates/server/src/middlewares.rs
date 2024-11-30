use axum::{
    extract::{MatchedPath, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tracing::info;

pub async fn logger(req: Request, next: Next) -> Result<Response, StatusCode> {
    let resource = req
        .extensions()
        .get::<MatchedPath>()
        .map_or_else(|| req.uri().path(), |path| path.as_str());
    let resource_method = req.method().as_str();

    info!("Request completed: {} {} ", resource_method, resource);
    return Ok(next.run(req).await);
}
