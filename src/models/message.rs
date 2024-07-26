#[derive(Debug)]
pub struct RawMessage {
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub message_id: String,
    pub sender: String,
    pub timestamp: String,
    pub content: String,
}