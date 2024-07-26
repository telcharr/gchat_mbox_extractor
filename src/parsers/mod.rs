mod mbox_parser;
mod html_parser;
mod message_parser;

pub use mbox_parser::split_mbox_entries;
pub use html_parser::extract_html_and_attachments;
pub use message_parser::{split_messages, parse_message};