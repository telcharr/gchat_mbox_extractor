use crate::models::{RawMessage, Message};
use regex::Regex;
use html_escape::decode_html_entities;

pub fn split_messages(html: &str) -> Vec<RawMessage> {
    let mut messages = Vec::new();
    let split_pattern = "<div data-id=\"";
    let parts: Vec<&str> = html.split(split_pattern).collect();

    for part in parts.iter().skip(1) {
        let message = format!("{}{}", split_pattern, part);
        messages.push(RawMessage {
            content: message,
        });
    }

    messages
}

pub fn parse_message(raw_content: &str) -> Option<Message> {
    let id_regex = Regex::new(r#"<div data-id="([^"]+)""#).ok()?;
    let message_id = id_regex.captures(raw_content)?.get(1)?.as_str().to_string();

    let sender_regex = Regex::new(r#"<span style="font-weight:700">(.*?)</span>"#).ok()?;
    let sender = sender_regex.captures(raw_content)?.get(1)?.as_str().trim().to_string();

    let timestamp_regex = Regex::new(r#"(?s)<div><span style="font-weight:700">.*?</span>(.*?)</div>"#).ok()?;
    let timestamp = timestamp_regex.captures(raw_content)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim())
        .filter(|&s| !s.is_empty() && !s.contains("Reply"))
        .unwrap_or("No timestamp")
        .to_string();

    let content_regex = Regex::new(r#"white-space:pre-wrap;width:100%">(.*?)</div>"#).ok()?;
    let content = content_regex.captures(raw_content)?.get(1)?.as_str().trim();

    let clean_content = clean_message_content(content);

    Some(Message {
        message_id,
        sender,
        timestamp,
        content: clean_content,
    })
}

fn clean_message_content(content: &str) -> String {
    let tag_regex = Regex::new(r"<[^>]+>").unwrap();
    let without_tags = tag_regex.replace_all(content, "");

    let decoded = decode_html_entities(&without_tags).to_string();

    let reply_regex = Regex::new(r"^\s*\d+\s+Reply\s*").unwrap();
    let without_reply = reply_regex.replace(&decoded, "");

    without_reply.trim().to_string()
}