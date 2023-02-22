//! impl of [`crate::extensions::Extensions::RoomV1`] required datatypes.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritRoomFromV1 {
    pub room_id: String,
    pub event_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomJoinRuleV1 {
    Public,
    Private,
    Invite,
    Knock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomJoinRestrictionV1 {
    ConnectedWallet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionMethod {
    /// aes256 with password.
    AES256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessagePartV1 {
    pub text: String,
    /// encryption method used to encrypt the text.
    /// if not set, the text is not encrypted.
    /// if set, the text is encrypted.
    pub encryption_method: Option<EncryptionMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionMessagePartV1 {
    /// the event id.
    pub react_to: String,
    /// the reaction emoji.
    pub reaction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessagePartV1 {
    Text(TextMessagePartV1),
    Reaction(ReactionMessagePartV1),
    Other(BTreeMap<String, Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageV1 {
    /// the event id.
    pub reply_to: Option<String>,
    /// pin the message to the top of the room.
    pub pin: bool,
    /// the message parts.
    pub message_parts: Vec<MessagePartV1>,
}
