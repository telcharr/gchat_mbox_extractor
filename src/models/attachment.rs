/// Represents an attachment.
///
/// # Fields
///
/// * `content_type` - A String containing the MIME type of the attachment.
/// * `filename` - A String representing the name of the attachment file.
/// * `content` - A String containing the base64-encoded content of the attachment.
#[derive(Debug)]
pub struct Attachment {
    pub content_type: String,
    pub filename: String,
    pub content: String,
}