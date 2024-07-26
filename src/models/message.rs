#[derive(Debug)]
pub struct RawMessage {
    pub content: String,
}

#[derive(Debug)]
pub struct Message {
    pub message_id: String,
    pub sender: String,
    pub timestamp: String,
    pub content: String,
}