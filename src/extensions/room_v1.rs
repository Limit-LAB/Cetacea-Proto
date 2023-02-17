//! impl of [`crate::extensions::Extensions::RoomV1`] required datatypes.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InheritRoomFromV1 {
    room_id: String,
    event_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoomJoinRuleV1 {
    Public,
    Private,
    Invite,
    Knock,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoomJoinRestrictionV1 {
    ConnectedWallet,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EncryptionMethod {
    /// aes256 with password.
    AES256,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextMessagePartV1 {
    text: String,
    /// encryption method used to encrypt the text.
    /// if not set, the text is not encrypted.
    /// if set, the text is encrypted.
    encryption_method: Option<EncryptionMethod>,
}

pub struct ReactionMessagePartV1 {
    /// the event id.
    react_to: String,
    /// the reaction emoji.
    reaction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageV1 {
    /// the event id.
    reply_to: Option<String>,
    /// pin the message to the top of the room.
    pin: bool,
    /// the message parts.
    message_parts: Vec<serde_json::Value>,
}
