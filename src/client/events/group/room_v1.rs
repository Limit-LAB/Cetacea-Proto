//! impl of [`crate::extensions::Extensions::RoomV1`] module.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    client::events::ClientEventSendHeader,
    extensions::{room_v1::*, Extensions},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequestV1 {
    header: ClientEventSendHeader,
    extensions: Extensions,
    extensions_data: BTreeMap<Extensions, String>,
    inherit_from: Option<InheritRoomFromV1>,
    room_info: RoomInfoV1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomResponseV1 {}

/// The sync room request.
/// one of `from_event_id` `to_event_id` and `limit` must be set.
/// `from_event_id` and `to_event_id` are exclusive.
/// `limit` is the max number of events to return.
/// if only `from_event_id` is set, the server will return the events after
/// `from_event_id`. if only `to_event_id` is set, the server will return the
/// events before `to_event_id`. if both `from_event_id` and `to_event_id` are
/// set, the server will return the events between `from_event_id` and
/// `to_event_id`. if `limit` is set, the server will return at most recent
/// events by limit.
///
/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | yes |
/// | require auth | yes |
///
/// may return [`crate::error_code::ErrorCode::InvalidEventId`] if the event id
/// is invalid.
///
/// POST `/client/room_v1/sync`
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRoomRequestV1 {
    room_id: String,
    from_event_id: Option<String>,
    to_event_id: Option<String>,
    limit: Option<u32>,
}

/// The sync room response.
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRoomResponseV1 {
    messages: Vec<MessageV1>,
}

/// The send messages request.
///
/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | yes |
/// | require auth | yes |
///
/// may return [`crate::error_code::ErrorCode::InvalidEventId`] if the event id
/// is invalid.
/// POST `/client/room_v1/send_messages`
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessagesRequestV1 {
    room_id: String,
    messages: Vec<MessageV1>,
}

/// The send messages response.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessagesResponseV1 {}

/// for last shown in room
/// and for read marker
/// mainly for streaming
///
/// POST `/client/room_v1/notify`
#[derive(Debug, Serialize, Deserialize)]
pub struct NotifyRequestV1 {
    notify_type: NotifyEventsV1,
}
