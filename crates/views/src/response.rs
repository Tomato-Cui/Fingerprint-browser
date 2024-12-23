use std::fmt::Debug;

const CODE_FAIL: i8 = 0;
const CODE_SUCCESS: i8 = 1;
const EMPTY_MESSAGE: &str = "empty message.";

#[derive(Debug, serde::Serialize)]
pub struct AppResponse<T> {
    pub code: i8,
    pub message: String,
    pub data: Option<T>,
}

impl<T> AppResponse<T> {
    pub fn default() -> Self {
        AppResponse {
            code: CODE_SUCCESS,
            message: EMPTY_MESSAGE.to_string(),
            data: None,
        }
    }
    pub fn fail(msg: Option<String>) -> Self {
        AppResponse {
            code: CODE_FAIL,
            message: msg.unwrap_or(EMPTY_MESSAGE.to_string()),
            data: None,
        }
    }
    pub fn success(msg: Option<String>, data: Option<T>) -> Self {
        AppResponse {
            code: CODE_SUCCESS,
            message: msg.unwrap_or(EMPTY_MESSAGE.to_string()),
            data,
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
}
