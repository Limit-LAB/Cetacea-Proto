#![doc = include_str!("../../docs/control.md")]

use serde::{Deserialize, Serialize};

use super::id::*;

/// An user in server A want to join a channel in server B.
#[derive(Debug, Serialize, Deserialize)]
pub struct CrossJoinChannelRequest {
    /// the server name of the server that the user in.
    from: ServerName,
    /// the server name of the server that the channel in.
    to: ServerName,
    channel: ChannelId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrossJoinChannelResponse {
    /// status info (send success or fail).
    status: String,
    // info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPublicChannelRequest {
    /// the server name of the server that the user in.
    origin: ServerName,
    /// the user id sent by the application.
    sender: UserId,

    channel: ChannelId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPublicChannelResponse {
    /// status info (send success or fail).
    status: String,
    // info: String,
}
