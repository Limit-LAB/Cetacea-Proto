use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryResponse {
    pub name: String,
    pub key: String,
    pub nodes: Vec<SocketAddr>,
}
