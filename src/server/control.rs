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
