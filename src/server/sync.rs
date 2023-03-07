#![doc = include_str!("../../docs/server_sync.md")]

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Message {
  // todo
}

#[derive(Debug, Serialize, Deserialize)]
struct UserId {
  // uuid
}


#[derive(Debug, Serialize, Deserialize)]
struct ChannelId {
  // uuid
}


#[derive(Debug, Serialize, Deserialize)]
enum SubjectId {
  User(UserId),
  ChannelId(ChannelId),
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageId {
  // uuid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePushRequest {
  prev_message_id: MessageId,
  message: Message,
  sender: UserId,
  reciver: SubjectId,
}
