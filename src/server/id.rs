use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UUID();


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerName(pub String);


#[derive(Debug, Serialize, Deserialize)]
pub struct UserId (pub UUID);


#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelId (pub UUID);


#[derive(Debug, Serialize, Deserialize)]
pub enum SubjectId {
  User(UserId),
  ChannelId(ChannelId),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageId (pub UUID);
