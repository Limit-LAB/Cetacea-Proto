use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod group;
pub mod sync;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientEventSendHeader {
    event_id: String,
    event_type: String,
    previous_event_id: Option<String>,
}

/// The client event get request.
///
/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | yes |
/// | require auth | yes |
///
/// may return [`crate::error_code::ErrorCode::InvalidEventId`] if the event id
/// is invalid.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientEventGetRequest {
    pub event_id: String,
}

/// The client event get response.
pub struct ClientEventGetResponse {
    pub event_id: String,
    pub event_type: String,
    pub previous_event_id: Option<String>,
}
