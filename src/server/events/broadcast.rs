use std::collections::BTreeMap;

use serde_json::Value;

pub struct BroadCastEvent {
    pub server_id: String,
    pub event_id: String,
    pub previous_event_id: Option<String>,
    // TODO: body data struct definition
    pub event_body: BTreeMap<String, Value>,
}
