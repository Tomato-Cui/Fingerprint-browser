use axum::{
    extract::{MatchedPath, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tokio::time::Instant;
use tracing::info;

pub async fn logger(req: Request, next: Next) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let resource_path = req.extensions().get::<MatchedPath>().map_or_else(
        || req.uri().path().to_string(),
        |path| path.as_str().to_string(),
    );
    let method = req.method().to_string();

    info!("Request started: {} {}", method, resource_path);

    let response = next.run(req).await;

    let latency = start.elapsed();
    let status = response.status().as_u16();
    info!(
        "Request completed: {} {} status={} latency={:?}",
        method, resource_path, status, latency
    );

    Ok(response)
}
