//! pubkey login module
//! bind to [`crate::extensions::Extensions::PubkeyLogin`]
//!
//! | limit | yes/no |
//! | --- | --- |
//! | rate limit   | yes |
//! | require auth | no  |
//!
//! may return [`crate::error_code::ErrorCode::UnsupportPubkeyLogin`] if the
//! pubkey login is not supported.

use serde::{Deserialize, Serialize};

use crate::{RequestMessage, Response};

/// The pubkey login request.
/// should use a valid Pubkey
///
/// may return [`crate::error_code::ErrorCode::InvalidPubkey`] if the pubkey is
/// invalid.
#[derive(Debug, Serialize, Deserialize)]
pub struct PubkeyLoginRequest {
    pub header: super::CommonLoginRequestHeader,
    pub pubkey: String,
    pub sign_jwt_duration: Option<u64>,
}

impl RequestMessage for PubkeyLoginRequest {}

/// The pubkey login response.
///
/// may return [`crate::error_code::ErrorCode::InvalidPubkey`] if the pubkey is
/// invalid.
#[derive(Debug, Serialize, Deserialize)]
pub struct PubkeyLoginResponse {
    pub jwt_token: String,
}

impl Response for PubkeyLoginResponse {}
