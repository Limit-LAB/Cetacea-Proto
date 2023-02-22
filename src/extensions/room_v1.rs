//! impl of [`crate::extensions::Extensions::RoomV1`] required datatypes.

use serde::{Deserialize, Serialize, Serializer};

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

#[derive(Debug, Serialize, Deserialize)]
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
    message_parts: Vec<MessagePartV1>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessagePartV1 {
    Supported(MessagePartV1Inner),
    Other {
        #[serde(rename = "type")]
        type_: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessagePartV1Inner {
    Text(TextMessagePartV1),
    Reaction(ReactionMessagePartV1),
}

#[test]
fn test_extensibility() {
    #[derive(Debug, Serialize, Deserialize)]
    struct DemoMessage {
        a: String,
        b: Option<Vec<u8>>,
        c: Result<(), String>,
    }
    let demo = DemoMessage {
        a: "hello".to_string(),
        b: None,
        c: Ok(()),
    };
    let messagev1 = MessageV1 {
        reply_to: None,
        pin: false,
        message_parts: vec![
            MessagePartV1::Other {
                type_: "demo".to_string(),
                data: serde_json::to_value(demo).unwrap(),
            },
            MessagePartV1::Supported(MessagePartV1Inner::Text(TextMessagePartV1 {
                text: "hello".to_string(),
                encryption_method: None,
            })),
            MessagePartV1::Supported(MessagePartV1Inner::Reaction(ReactionMessagePartV1 {
                react_to: "123".to_string(),
                reaction: "👍".to_string(),
            })),
        ],
    };

    let rmp = rmp_serde::to_vec_named(&messagev1).unwrap();
    println!("{:?}", rmp);
    let messagev1: serde_json::Value = rmp_serde::from_read_ref(&rmp).unwrap();
    println!("{:#?}", messagev1);
    let messagev1: MessageV1 = rmp_serde::from_read_ref(&rmp).unwrap();
    println!("{:#?}", messagev1);
}
