//! jwt login module
//! bind to [`crate::extensions::Extensions::JWTLogin`]
//!
//! | limit | yes/no |
//! | --- | --- |
//! | rate limit   | yes |
//! | require auth | no  |
//!
//! may return [`crate::error_code::ErrorCode::UnsupportJWTLogin`] if the jwt
//! login is not supported.

use serde::{Deserialize, Serialize};

use crate::{RequestMessage, Response};

/// The jwt login request.
/// should use a valid JWT token.
///
/// may return [`crate::error_code::ErrorCode::InvalidJWTToken`] if the jwt
/// token is invalid.
///
/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | yes |
/// | require auth | no |
///
/// POST `/auth/jwt_v1/login`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JwtLoginRequestV1 {
    #[serde(flatten)]
    pub header: Option<super::CommonLoginRequestHeader>,
    pub jwt_token: String,
}

impl RequestMessage for JwtLoginRequestV1 {}

/// The jwt login response.
///
/// may return [`crate::error_code::ErrorCode::InvalidJWTToken`] if the jwt
/// token is invalid.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JwtLoginResponseV1 {}

impl Response for JwtLoginResponseV1 {}
