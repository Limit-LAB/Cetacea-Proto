#![feature(associated_type_defaults)]
#![feature(const_type_name)]

pub mod client;
pub mod server;

pub mod error;
pub mod extensions;

mod util;

use error::Error;
use serde::{de::DeserializeOwned, Serialize};

use crate::{error::ErrorType, sealed::Sealed};

mod sealed {
    pub trait Sealed {}
}

/// Marker trait for request
pub trait Request: Serialize + Sealed {}

/// Marker trait for response
pub trait Response: Serialize + DeserializeOwned + Sealed {}

pub type ResponseMessage<T> = Result<T, ErrorType>;

macro_rules! impl_request {
    ($($t:ty),*) => {
        $(
            impl $crate::Sealed for $t {}
            impl $crate::Request for $t {}
        )*
    };
}

macro_rules! impl_response {
    ($($t:ty),*) => {
        $(
            impl $crate::Sealed for $t {}
            impl $crate::Response for $t {}
        )*
    };
}

pub(crate) use impl_request;
pub(crate) use impl_response;

/// After connected, the client will send this request to the server
pub struct StreamRequestRequest<T: Request> {
    pub request: T,
}

/// Response of [`StreamRequestRequest`],
pub struct StreamRequestResponse {
    pub request_id: String,
    pub fail_reason: Option<ErrorType>,
    pub count: u32,
}

/// Subsequent messages requested by [`StreamRequestResponse`]. The server will
/// continuously send this message to the client
pub struct StreamResponse<T: Response> {
    pub request_id: String,
    pub count: u32,
    pub response: T,
}
