/// Represents a raw, unparsed message from the .mbox file.
///
/// # Fields
///
/// * `content` - A String containing the raw, unparsed HTML of the message.
#[derive(Debug)]
pub struct RawMessage {
    pub content: String,
}

/// Represents a parsed message from the .mbox file.
///
/// # Fields
///
/// * `message_id` - A String containing the unique identifier of the message.
/// * `sender` - A String containing the name or email address of the message sender.
/// * `timestamp` - A String containing the timestamp of when the message was sent.
/// * `content` - A String containing the parsed content of the message.
#[derive(Debug, Clone)]
pub struct Message {
    pub message_id: String,
    pub sender: String,
    pub timestamp: String,
    pub content: String,
}