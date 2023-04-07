use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::util::cfg_wasm;

#[repr(u32)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ErrorCode {
    // Use none-zero value to avoid uninitialized memory or bad input being treated as proper
    // value
    UNKNOWN             = u32::MAX,

    InvalidEventId      = 1,
    UnsupportedWallet   = 2,
    InvalidJWTToken     = 3,
    UnsupportedJWTLogin = 4,
    InvalidPubkey       = 5,
    UnsupportedPubkeyLogin = 6,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Invalid message type: expect {expect}, got {got}")]
    InvalidMessageType { expect: &'static str, got: String },
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Clone, Debug, Error)]
pub struct ErrorType {
    pub error_code: ErrorCode,
    pub error_message: Vec<String>,
}

cfg_wasm! {
    impl From<ErrorType> for wasm_bindgen::JsValue {
        fn from(val: ErrorType) -> Self {
            serde_wasm_bindgen::to_value(&val).unwrap()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ErrorMessageDisplayWrapper<'a, T: 'a>(pub &'a [T]);

impl<'a, T: std::fmt::Display> std::fmt::Display for ErrorMessageDisplayWrapper<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.len() {
            0 => {}
            1 => write!(f, ": {}", self.0[0])?,
            _ => {
                let mut first = true;
                write!(f, ": [")?;

                for item in self.0 {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}(error code {}){}",
            self.error_code,
            self.error_code as u32,
            ErrorMessageDisplayWrapper(&self.error_message)
        )
    }
}

#[test]
fn test_error_code() {
    let err = ErrorType {
        error_code: ErrorCode::InvalidEventId,
        error_message: vec!["test".to_string(), "oops".to_string()],
    };

    assert_eq!(
        err.to_string(),
        "InvalidEventId(error code 1): [test, oops]"
    );

    let err = ErrorType {
        error_code: ErrorCode::InvalidEventId,
        error_message: vec!["test".to_string()],
    };

    assert_eq!(err.to_string(), "InvalidEventId(error code 1): test");

    let err = ErrorType {
        error_code: ErrorCode::InvalidEventId,
        error_message: vec![],
    };

    assert_eq!(err.to_string(), "InvalidEventId(error code 1)");
}
