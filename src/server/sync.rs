#![doc = include_str!("../../docs/server_sync.md")]

use serde::{Deserialize, Serialize};

use super::id::*;

#[derive(Debug, Serialize, Deserialize)]
struct Signature(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePushRequest {
    // prev_message_id: MessageId,
    signature: Signature,
    message: String,
    sender: UserId,
    receiver: SubjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePushResponse {
    // prev_message_id: MessageId,
    // signature: Signature,
    // origin: ServerName,
    // message: String,
    // sender: UserId,
    // receiver: SubjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMissingMessageRequest {
    origin: ServerName,
    receiver: UserId,
    sender: SubjectId,
    // maybe timestamp
    lower_message: MessageId,
    upper_message: Option<MessageId>,
    // event_type: EventType
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMissingMessageResponse {
    message: String,
}
