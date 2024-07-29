use super::{Attachment, Message};

/// Represents an entry in an .mbox file.
///
/// # Fields
///
/// * `headers` - A String containing the headers of the .mbox entry.
/// * `html_body` - A String containing the HTML body of the .mbox entry.
/// * `attachments` - A Vector of Attachment structs representing any attachments in the entry.
/// * `messages` - A Vector of Message structs representing the individual messages in the entry.
#[derive(Debug)]
pub struct MboxEntry {
    pub headers: String,
    pub html_body: String,
    pub attachments: Vec<Attachment>,
    pub messages: Vec<Message>,
}