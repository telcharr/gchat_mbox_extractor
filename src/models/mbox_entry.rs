use super::{Attachment, Message};

#[derive(Debug)]
pub struct MboxEntry {
    pub headers: String,
    pub html_body: String,
    pub attachments: Vec<Attachment>,
    pub messages: Vec<Message>,
}