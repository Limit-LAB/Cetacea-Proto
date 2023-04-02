use serde::{Deserialize, Serialize};

pub mod group;
pub mod sync;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientEventSendHeader {
    event_id: String,
    event_type: String,
    previous_event_id: Option<String>,
}

/// The client event point get request.
///
/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | yes |
/// | require auth | yes |
///
/// may return [`crate::error_code::ErrorCode::InvalidEventId`] if the event id
/// is invalid.
///
/// POST `/client/events_v1/get`
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientEventGetRequestV1 {
    pub event_id: String,
}

/// The client event get response.
pub struct ClientEventGetResponseV1 {
    pub event_id: String,
    pub event_type: String,
    pub previous_event_id: Option<String>,
}
