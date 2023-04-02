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
/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | yes |
/// | require auth | no |
///
/// may return [`crate::error_code::ErrorCode::InvalidPubkey`] if the pubkey is
/// invalid.
/// POST `/auth/pubkey_v1/login`
#[derive(Debug, Serialize, Deserialize)]
pub struct PubkeyLoginRequestV1 {
    pub header: super::CommonLoginRequestHeader,
    pub pubkey: String,
    pub sign_jwt_duration: Option<u64>,
}

impl RequestMessage for PubkeyLoginRequestV1 {}

/// The pubkey login response.
///
/// may return [`crate::error_code::ErrorCode::InvalidPubkey`] if the pubkey is
/// invalid.
#[derive(Debug, Serialize, Deserialize)]
pub struct PubkeyLoginResponseV1 {
    pub jwt_token: String,
}

impl Response for PubkeyLoginResponseV1 {}
