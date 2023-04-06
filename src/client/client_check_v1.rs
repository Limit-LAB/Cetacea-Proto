use serde::{Deserialize, Serialize};

use crate::{
    extensions::{CheckSupportedExtensionsRequest, CheckSupportedExtensionsResponse},
    RequestMessage, Response,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCheckServerRequestV1 {
    pub server_addr: String,
    pub check_extensions: CheckSupportedExtensionsRequest,
}
impl RequestMessage for ClientCheckServerRequestV1 {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCheckServerResponseV1 {
    pub server_version: String,
    pub server_name: String,
    pub server_description: String,
    pub server_latency: u64,
    pub supported_extensions: CheckSupportedExtensionsResponse,
}
impl Response for ClientCheckServerResponseV1 {}
