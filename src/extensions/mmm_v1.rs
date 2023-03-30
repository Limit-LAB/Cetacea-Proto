use serde::{Deserialize, Serialize};

use crate::{extensions::room_v1::MessagePartV1, impl_message_v1};

// video
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoMessagePartV1 {
    #[serde(flatten)]
    pub file: FileMessagePartV1,

    pub thumbnail_url: Option<String>,
}
impl_message_v1!(VideoMessagePartV1);

// audio
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMessagePartV1 {
    #[serde(flatten)]
    file: FileMessagePartV1,
}
impl_message_v1!(AudioMessagePartV1);

// location
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMessagePartV1 {
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
    pub description: String,
}
impl_message_v1!(LocationMessagePartV1);

// file
#[derive(Debug, Serialize, Deserialize)]
pub struct FileMessagePartV1 {
    pub url: String,
    pub mime_type: Option<String>,
    pub name: String,
    pub size: u64,
}
impl_message_v1!(FileMessagePartV1);
