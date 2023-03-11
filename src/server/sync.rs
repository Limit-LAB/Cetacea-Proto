#![doc = include_str!("../../docs/server_sync.md")]

use serde::{Deserialize, Serialize};

use super::id::*;


#[derive(Debug, Serialize, Deserialize)]
struct Message {
  // todo
}




#[derive(Debug, Serialize, Deserialize)]
struct Signature (pub String);


#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePushRequest {
  prev_message_id: MessageId,
  signature: Signature,
  message: Message,
  sender: UserId,
  reciver: SubjectId,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePushResponse {
  prev_message_id: MessageId,
  signature: Signature,
  origin: ServerName,
  message: Message,
  sender: UserId,
  reciver: SubjectId,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetMissingMessageRequest {
  origin: ServerName,
  sender: UserId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMissingMessageResponse {
  message_list: Vec<Message>,
}