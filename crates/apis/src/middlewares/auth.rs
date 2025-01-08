use super::CurrentUser;
use axum::{
    extract::{MatchedPath, Request},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use services::{operation_log, permission};

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

    let resource = &format!("{}+{}", resource, resource_method.to_ascii_uppercase());
    let ok = permission::can_check_permission(resource).await;

    if let Ok(ok) = ok {
        if !ok {
            req.extensions_mut().insert(CurrentUser { user_uuid: uuid });
            return Ok(next.run(req).await);
        }
    }

    let ok = permission::check_permission(&uuid, resource)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if !ok {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let _ = operation_log::record_operation_log(&uuid, resource);

    req.extensions_mut().insert(CurrentUser { user_uuid: uuid });
    return Ok(next.run(req).await);
}
