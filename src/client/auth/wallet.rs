//! The wallet module contains the wallet API.
//! bind to [`crate::extensions::Extensions::WalletLogin`]
//!
//! | limit | yes/no |
//! | --- | --- |
//! | rate limit   | yes |
//! | require auth | no  |
//!
//! may return [`crate::error_code::ErrorCode::UnsupportWallet`] if the wallet
//! login is not supported.

use serde::{Deserialize, Serialize};

/// The wallet login request.
/// should use a valid ETH wallet address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletLoginRequest {
    pub header: super::CommonLoginRequestHeader,
}
