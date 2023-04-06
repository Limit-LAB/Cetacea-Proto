#![feature(associated_type_defaults)]
#![feature(const_type_name)]

pub mod client;
pub mod server;

pub mod error_code;
pub mod extensions;

use error_code::ErrorCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen::JsValue;

pub trait RequestMessage: Serialize {}

pub trait Response: Serialize + DeserializeOwned {}

pub type ResponseMessage<T> = Result<T, ErrorType>;

/// first send this request to ws
pub struct StreamRequestRequest<T: RequestMessage> {
    pub request: T,
}

/// afer send [`StreamRequestRequest`], the server will send this response
pub struct StreamRequestResponse {
    pub request_id: String,
    pub fail_reason: Option<ErrorType>,
    pub count: u32,
}

/// after receiving [`StreamRequestResponse`], the client will continuesly
/// getting
pub struct StreamResponse<T: Response> {
    pub request_id: String,
    pub count: u32,
    pub response: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, Error)]
pub struct ErrorType {
    pub error_code: ErrorCode,
    pub error_message: String,
}

impl Into<JsValue> for ErrorType {
    fn into(self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ErrorType {{ error_code: {:?}, error_message: {} }}",
            self.error_code, self.error_message
        )
    }
}
