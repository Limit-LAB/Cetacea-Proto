#![doc = include_str!("../../docs/server_sync.md")]

use serde::{Deserialize, Serialize};

use super::id::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
struct Signature(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePushRequest {
    // prev_message_id: MessageId,
    /// signature of this message (using the key issued by the server)
    signature: Signature,
    /// the message the user wants to send
    message: String,
    /// user who wants to send a message
    sender: UserId,
    /// receiver id (channel id or user id)
    receiver: SubjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePushResponse {
    /// status info (send success or fail)
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMissingMessageRequest {
    /// the server name of the server that the user in
    origin: ServerName,
    /// users who want to get message
    receiver: UserId,
    /// the subject of the message to the user
    sender: SubjectId,
    /// range of messages to get
    // maybe timestamp
    lower_message: MessageId,
    upper_message: Option<MessageId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMissingMessageResponse {
    message: String,
}
