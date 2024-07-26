use crate::models::{MboxEntry, Message, RawMessage};
use crate::parsers::{extract_html_and_attachments, split_messages, parse_message};
use crate::utils::decode_quoted_printable;
use std::error::Error;
use rayon::prelude::*;
use core::str::Lines;

pub fn split_mbox_entries(content: &str) -> Result<Vec<MboxEntry>, Box<dyn Error>> {
    let parts: Vec<&str> = content.split("\nFrom ").collect();

    let entries: Vec<MboxEntry> = parts.par_iter().skip(1).filter_map(|part| {
        let mut lines: Lines = part.lines();
        let mut headers: String = String::new();
        let mut body: String = String::new();
        let mut in_headers: bool = true;

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
        let decoded_html: String = decode_quoted_printable(&html_body).ok()?;
        let raw_messages: Vec<RawMessage> = split_messages(&decoded_html);
        let messages: Vec<Message> = raw_messages.par_iter()
            .filter_map(|rm: &RawMessage| parse_message(&rm.content))
            .collect();

        Some(MboxEntry {
            headers: headers.trim().to_string(),
            html_body,
            attachments,
            messages,
        })
    }).collect();

    Ok(entries)
}