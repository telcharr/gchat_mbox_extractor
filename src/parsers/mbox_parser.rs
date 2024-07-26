use crate::models::MboxEntry;
use crate::parsers::{extract_html_and_attachments, split_messages, parse_message};
use crate::utils::decode_quoted_printable;
use std::error::Error;

pub fn split_mbox_entries(content: &str) -> Result<Vec<MboxEntry>, Box<dyn Error>> {
    let mut entries = Vec::new();
    let parts: Vec<&str> = content.split("\nFrom ").collect();

    for part in parts.iter().skip(1) {
        let lines = part.lines();
        let mut headers = String::new();
        let mut body = String::new();
        let mut in_headers = true;

        for line in lines {
            if in_headers {
                if line.is_empty() {
                    in_headers = false;
                } else {
                    headers.push_str(line);
                    headers.push('\n');
                }
            } else {
                body.push_str(line);
                body.push('\n');
            }
        }

        let (html_body, attachments) = extract_html_and_attachments(&body);
        let decoded_html = decode_quoted_printable(&html_body)?;
        let raw_messages = split_messages(&decoded_html);
        let messages = raw_messages.iter().filter_map(|rm| parse_message(&rm.content)).collect();

        entries.push(MboxEntry {
            headers: headers.trim().to_string(),
            html_body,
            attachments,
            messages,
        });
    }

    Ok(entries)
}