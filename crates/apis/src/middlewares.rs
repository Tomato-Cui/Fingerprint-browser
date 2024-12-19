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
    pub id: u32,
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

    let id = match commons::encryption::verify_token(token_str) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // 进行权限校验，只有用户存在该资源才有权进行访问。
    // if let Ok(exist) = lc_services::auth(&uuid, resource, resource_method).await {
    //     if exist {
    //         req.extensions_mut().insert(CurrentUser { uuid });
    //         return Ok(next.run(req).await);
    //     }
    // }
    // return Err(StatusCode::UNAUTHORIZED);
    req.extensions_mut().insert(CurrentUser { id: id as u32 });
    return Ok(next.run(req).await);
}
