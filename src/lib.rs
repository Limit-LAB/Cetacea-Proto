#![feature(associated_type_defaults)]
#![feature(const_type_name)]

pub mod client;
pub mod server;

pub mod error_code;
pub mod extensions;

use error_code::ErrorCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

pub trait RequestMessage: Serialize {}

pub trait Response: Serialize + DeserializeOwned {}

pub type ResponseMessage<T> = Result<T, ErrorType>;

pub struct StreamRequest<T: RequestMessage> {
    pub request_id: String,
    pub request: T,
}

pub struct StreamResponse<T: Response> {
    pub request_id: String,
    pub count: u32,
    pub response: ResponseMessage<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Error)]
pub struct ErrorType {
    pub error_code: ErrorCode,
    pub error_message: String,
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
