//! impl of [`crate::extensions::Extensions::RoomV1`] required datatypes.

use serde::{Deserialize, Serialize};

use super::user_v1::UserHeaderV1;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfoV1 {
    pub room_id: String,
    pub room_version: String,
    pub name: String,
    pub description: String,
    pub avatar: String,
    pub join_rule: RoomJoinRuleV1,
    pub join_restriction: Vec<RoomJoinRestrictionV1>,
    pub created_at: u64,
    pub members: Vec<UserHeaderV1>,
    pub admins: Vec<UserHeaderV1>,
    pub banned_members: Vec<UserHeaderV1>,
    pub default_admin_abilities: Vec<AdminAbilitiesV1>,
    pub default_member_abilities: Vec<AdminAbilitiesV1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AdminAbilitiesV1 {
    // admin
    ChangeRoomName,
    ChangeRoomDescription,
    ChangeRoomAvatar,
    ChangeRoomJoinRule,
    ChangeRoomJoinRestriction,
    ChangeRoomAdmins,
    BanRoomMembers,
    UnbanRoomMembers,
    ChangeRoomDefaultAdminAbilities,
    ChangeRoomDefaultMemberAbilities,
    AcceptKnock,

    // normal member
    InviteRoomMembers,
    SendMessages,
    ReactMessages,
}

/// fork a room from another room. by the location of event id.
/// for example, a room server is dead, or some part of the room members are not
/// happy with the room server. the forked room will only have the message
/// without people or settings.
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
pub struct MessagePartV1 {
    #[serde(rename = "type")]
    type_: String,
    #[serde(flatten)]
    data: serde_json::Value,
}

#[macro_export]
macro_rules! impl_message_v1 {
    ($t:ty) => {
        impl TryFrom<MessagePartV1> for $t {
            type Error = anyhow::Error;

            fn try_from(value: MessagePartV1) -> Result<Self, Self::Error> {
                if value.type_ != std::any::type_name::<Self>() {
                    return Err(anyhow::anyhow!("not a text message part"));
                }
                serde_json::from_value(value.data).map_err(|e| e.into())
            }
        }

        impl From<$t> for MessagePartV1 {
            fn from(t: $t) -> MessagePartV1 {
                MessagePartV1 {
                    type_: std::any::type_name::<$t>().to_string(),
                    data: serde_json::to_value(t).unwrap(),
                }
            }
        }
    };
}

/// for example you want to send a message only want some members to see.
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
impl_message_v1!(TextMessagePartV1);

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionMessagePartV1 {
    /// the event id.
    react_to: String,
    /// the reaction emoji.
    reaction: String,
}
impl_message_v1!(ReactionMessagePartV1);

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageV1 {
    /// the event id.
    reply_to: Option<String>,
    /// pin the message to the top of the room.
    pin: bool,
    /// the message parts.
    message_parts: Vec<MessagePartV1>,
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
            MessagePartV1 {
                type_: "demo".to_string(),
                data: serde_json::to_value(demo).unwrap(),
            },
            TextMessagePartV1 {
                text: "hello".to_string(),
                encryption_method: None,
            }
            .into(),
            ReactionMessagePartV1 {
                react_to: "123".to_string(),
                reaction: "👍".to_string(),
            }
            .into(),
        ],
    };

    let rmp = rmp_serde::to_vec_named(&messagev1).unwrap();
    println!("{:?}", rmp);
    let messagev1: serde_json::Value = rmp_serde::from_slice(&rmp).unwrap();
    println!("{:#?}", messagev1);
    let messagev1: MessageV1 = rmp_serde::from_slice(&rmp).unwrap();
    println!("{:#?}", messagev1);
}
