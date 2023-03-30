use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ErrorCode {
    UNKNOWN = 0,

    InvalidEventId,
    UnsupportWallet,
    InvalidJWTToken,
    UnsupportJWTLogin,
    InvalidPubkey,
    UnsupportPubkeyLogin,
}
