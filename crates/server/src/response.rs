use std::fmt::Debug;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

#[warn(dead_code)]
const CODE_FAIL: i8 = 1;
const CODE_SUCCESS: i8 = 0;
const EMPTY_MESSAGE: &str = "empty message.";

#[derive(Debug)]
pub struct AppStatusCode(StatusCode);

impl Serialize for AppStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.0.as_u16())
    }
}

#[derive(Debug, serde::Serialize)]
pub struct AppResponse<T> {
    pub code: i8,
    pub message: String,
    pub data: Option<T>,
    pub status_code: Option<AppStatusCode>,
}

impl<T> AppResponse<T> {
    pub fn default() -> Self {
        AppResponse {
            code: CODE_SUCCESS,
            message: EMPTY_MESSAGE.to_string(),
            data: None,
            status_code: Some(AppStatusCode(StatusCode::OK)),
        }
    }
    pub fn fail(msg: Option<String>) -> Self {
        AppResponse {
            code: CODE_FAIL,
            message: msg.unwrap_or(EMPTY_MESSAGE.to_string()),
            data: None,
            status_code: Some(AppStatusCode(StatusCode::BAD_REQUEST)),
        }
    }
    pub fn success(msg: Option<String>, data: Option<T>) -> Self {
        AppResponse {
            code: CODE_SUCCESS,
            message: msg.unwrap_or(EMPTY_MESSAGE.to_string()),
            data,
            status_code: Some(AppStatusCode(StatusCode::OK)),
        }
    }

    pub fn with_message(mut self, msg: String) -> Self {
        self.message = msg;
        self
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(AppStatusCode(status_code));
        self
    }
}

impl<T> IntoResponse for AppResponse<T>
where
    T: Debug + Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let mut content_json = json!({"code": self.code, "message": self.message});

        if let Some(data) = self.data {
            content_json = json!({"code": self.code, "message": self.message,"data": data});
        }

        (
            self.status_code
                .unwrap_or(AppStatusCode(StatusCode::BAD_GATEWAY))
                .0,
            Json(content_json),
        )
            .into_response()
    }
}
