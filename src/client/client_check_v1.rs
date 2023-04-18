use serde::{Deserialize, Serialize};

use crate::extensions::{CheckSupportedExtensionsRequest, CheckSupportedExtensionsResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCheckServerRequestV1 {
    pub server_addr: String,
    pub check_extensions: CheckSupportedExtensionsRequest,
}
crate::impl_request!(ClientCheckServerRequestV1);

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCheckServerResponseV1 {
    pub server_version: String,
    pub server_name: String,
    pub server_description: String,
    pub supported_extensions: CheckSupportedExtensionsResponse,
}
crate::impl_response!(ClientCheckServerResponseV1);
