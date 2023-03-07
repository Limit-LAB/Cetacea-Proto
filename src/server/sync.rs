#![doc = include_str!("../../docs/server_sync.md")]

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Message {
  // todo
}


#[derive(Debug, Serialize, Deserialize)]
struct UUID();


#[derive(Debug, Serialize, Deserialize)]
struct ServerName();


#[derive(Debug, Serialize, Deserialize)]
struct UserId (pub UUID, pub ServerName);


#[derive(Debug, Serialize, Deserialize)]
struct ChannelId (pub UUID, pub ServerName);


#[derive(Debug, Serialize, Deserialize)]
enum SubjectId {
  User(UserId),
  ChannelId(ChannelId),
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageId (pub UUID);


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
