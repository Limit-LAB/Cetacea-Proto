use serde::{Deserialize, Serialize};

pub mod jwt;
pub mod wallet;

/// The common login request header.

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonLoginRequestHeader {
    /// The device id.
    pub device_id: String,
    /// The user agent.
    pub user_agent: String,
}
