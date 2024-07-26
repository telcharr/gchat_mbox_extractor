#[derive(Debug)]
pub struct Attachment {
    pub content_type: String,
    pub filename: String,
    pub content: Vec<u8>,
}