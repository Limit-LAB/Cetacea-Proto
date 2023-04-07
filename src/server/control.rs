#![doc = include_str!("../../docs/control.md")]

use serde::{Deserialize, Serialize};

use super::id::*;

/// Request representing users joining a channel on another server
#[derive(Debug, Serialize, Deserialize)]
pub struct CrossJoinChannelRequest {
    /// the server name of the server that the user in
    from: ServerName,
    /// the server name of the server that the channel in
    to: ServerName,
    channel: ChannelId,
}

/// Response representing the result of a cross join channel request
#[derive(Debug, Serialize, Deserialize)]
pub struct CrossJoinChannelResponse {
    /// status info (send success or fail)
    status: String,
    // info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPublicChannelRequest {
    /// server name of user's homeserver
    from: ServerName,
    /// the user id sent by the application
    sender: UserId,

    channel: ChannelId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPublicChannelResponse {
    /// status info (send success or fail)
    status: String,
    // info: String,
}
