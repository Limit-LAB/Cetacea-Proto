//! The wallet module contains the wallet API.
//! bind to [`crate::extensions::Extensions::WalletLogin`]
//!
//! | limit | yes/no |
//! | --- | --- |
//! | rate limit   | yes |
//! | require auth | no  |
//!
//! may return [`crate::error_code::ErrorCode::UnsupportedWallet`] if the wallet
//! login is not supported.

use serde::{Deserialize, Serialize};

/// The wallet login request.
/// should use a valid ETH wallet address.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletLoginRequest {
    #[serde(flatten)]
    pub header: super::CommonLoginRequestHeader,
}
