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

/// The jwt login request.
/// should use a valid JWT token.
///
/// may return [`crate::error_code::ErrorCode::InvalidJWTToken`] if the jwt
/// token is invalid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtLoginRequest {
    pub header: super::CommonLoginRequestHeader,
    pub jwt_token: String,
}
