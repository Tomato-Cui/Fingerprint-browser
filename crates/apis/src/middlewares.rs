use axum::{
    extract::{MatchedPath, Request},
    http::{header, StatusCode},
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

#[derive(Clone)]
pub struct CurrentUser {
    pub user_uuid: String,
}

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let resource = req
        .extensions()
        .get::<MatchedPath>()
        .map_or_else(|| req.uri().path(), |path| path.as_str());
    let resource_method = req.method().as_str();

    let exist = services::resource_whitelist::exists(resource, resource_method).await;
    if let Ok(exist) = exist {
        if exist {
            return Ok(next.run(req).await);
        }
    }

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut bearer_token = auth_header.split_whitespace();
    bearer_token.next();
    let token_str = if let Some(token_str) = bearer_token.next() {
        token_str
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let uuid = match commons::encryption::verify_token(token_str) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    req.extensions_mut().insert(CurrentUser { user_uuid: uuid });
    return Ok(next.run(req).await);
}
