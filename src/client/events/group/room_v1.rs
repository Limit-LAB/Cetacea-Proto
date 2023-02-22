//! impl of [`crate::extensions::Extensions::RoomV1`] module.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::extensions::room_v1::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAliasRequestV1 {
    pub header: super::super::ClientEventSendHeader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAliasResponseV1 {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomRequestV1 {
    pub header: super::super::ClientEventSendHeader,
    pub extensions: crate::extensions::Extensions,
    pub extensions_data: BTreeMap<crate::extensions::Extensions, String>,
    pub inherit_from: Option<InheritRoomFromV1>,
    pub join_rule: RoomJoinRuleV1,
    pub join_restrictions: Option<RoomJoinRestrictionV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRoomRequestV1 {
    pub room_id: String,
    pub from_event_id: Option<String>,
    pub to_event_id: Option<String>,
    pub limit: Option<u32>,
}

/// The sync room response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRoomResponseV1 {}

/// The send messages request.
///
/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | yes |
/// | require auth | yes |
///
/// may return [`crate::error_code::ErrorCode::InvalidEventId`] if the event id
/// is invalid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessagesRequestV1 {
    pub room_id: String,
    pub messages: Vec<MessageV1>,
}

/// The send messages response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessagesResponseV1 {}

/// for last shown in room
/// and for read marker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginNotifyRequestV1 {
    pub header: super::super::ClientEventSendHeader,
}
