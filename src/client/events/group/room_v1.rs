//! impl of [`crate::extensions::Extensions::RoomV1`] module.
//! websocket route: `/ws/room_v1/<roomid>`

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    client::events::ClientEventSendHeader,
    extensions::{room_v1::*, Extensions},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequestV1 {
    #[serde(flatten)]
    header: ClientEventSendHeader,
    extensions: Extensions,
    extensions_data: BTreeMap<Extensions, String>,
    inherit_from: Option<InheritRoomFromV1>,
    room_info: RoomInfoV1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomResponseV1 {}

/// The sync room request.
///
/// `limit` is the max number of events to return. If only `from_event_id` is
/// set, the server will return the events after `from_event_id`. if only
/// `to_event_id` is set, the server will return the events before
/// `to_event_id`. if both `from_event_id` and `to_event_id` are set, the server
/// will return the events between `from_event_id` and `to_event_id`. .
///
/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | yes |
/// | require auth | yes |
///
/// may return [`crate::error_code::ErrorCode::InvalidEventId`] if the event id
/// is invalid.
///
/// POST `/client/room_v1/<roomid>/sync`
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRoomRequestV1 {
    room_id: String,
    #[serde(flatten)]
    sync_range: SyncRoomRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SyncRoomRange {
    From {
        from_event_id: String,
        limit: Option<u32>,
    },
    To {
        to_event_id: String,
        limit: Option<u32>,
    },
    Between {
        from_event_id: String,
        to_event_id: String,
    },
    Recent {
        limit: u32,
    },
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
/// POST `/client/room_v1/<roomid>/send_messages`
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessagesRequestV1 {
    room_id: String,
    messages: Vec<MessageV1>,
}

/// The send messages response.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessagesResponseV1 {
    event_id: String,
}

/// for last shown in room
/// and for read marker
/// mainly for streaming
///
/// POST `/client/room_v1/<roomid>/notify`
#[derive(Debug, Serialize, Deserialize)]
pub struct NotifyRequestV1 {
    notify_type: NotifyEventsV1,
}
