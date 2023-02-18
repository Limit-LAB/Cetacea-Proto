pub struct BroadCastEvent {
    pub server_id: String,
    pub event_id: String,
    pub previous_event_id: Option<String>,
    pub event_body: serde_json::Value,
}
