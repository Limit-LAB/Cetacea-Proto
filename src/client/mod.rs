#![doc = include_str!("../../docs/client.md")]

pub mod auth;
pub mod client_check_v1;
pub mod events;
pub mod user;

pub struct ClientCheckServerRequest {}

pub struct ClientCheckServerResponse {
    pub server_version: String,
    pub server_name: String,
    pub server_description: String,
}
