use serde::{Deserialize, Serialize};

use super::id::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinChannelRequest {
    origin: ServerName,
    sender: UserId,
    channel: ChannelId,
    // signature: Signature,
    // message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinChannelResponse {
    info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitingToChannelRequest {
    origin: ServerName,
    sender: UserId,
    membership: UserId,
    channel: ChannelId,
    // signature: Signature,
    // message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitingToChannelResponse {
    info: String,
    // todo
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeavingChannelRequest {
    origin: ServerName,
    sender: UserId,
    channel: ChannelId,
    // signature: Signature,
    // message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeavingChannelResponse {
    info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPublicChannelRequest {
    origin: ServerName,
    sender: UserId,
    channel: ChannelId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPublicChannelResponse {
    info: String,
}
