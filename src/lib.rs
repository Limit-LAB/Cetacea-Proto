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

pub trait ResponseMessage<T: Response>: Serialize + DeserializeOwned {
    type Response = Result<T, ErrorType>;
}

struct StreamRequest<T: RequestMessage> {
    request_id: String,
    request: T,
}

struct StreamResponse<T: Response> {
    request_id: String,
    count: u32,
    response: T,
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
