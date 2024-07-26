use std::borrow::Cow;
use crate::models::{RawMessage, Message};
use regex::{Captures, Match, Regex};
use html_escape::decode_html_entities;
use rayon::prelude::*;

pub fn split_messages(html: &str) -> Vec<RawMessage> {
    let split_pattern: &str = "<div data-id=\"";
    let parts: Vec<&str> = html.split(split_pattern).collect();

    parts.into_par_iter()
        .skip(1)
        .map(|part: &str| {
            RawMessage {
                content: format!("{}{}", split_pattern, part),
            }
        })
        .collect()
}

pub fn parse_message(raw_content: &str) -> Option<Message> {
    let id_regex: Regex = Regex::new(r#"<div data-id="([^"]+)""#).ok()?;
    let message_id: String = id_regex.captures(raw_content)?.get(1)?.as_str().to_string();

    let sender_regex: Regex = Regex::new(r#"<span style="font-weight:700">(.*?)</span>"#).ok()?;
    let sender: String = sender_regex.captures(raw_content)?.get(1)?.as_str().trim().to_string();

    let timestamp_regex: Regex = Regex::new(r#"(?s)<div><span style="font-weight:700">.*?</span>(.*?)</div>"#).ok()?;
    let timestamp: String = timestamp_regex.captures(raw_content)
        .and_then(|cap: Captures| cap.get(1))
        .map(|m: Match| m.as_str().trim())
        .filter(|&s: &str| !s.is_empty() && !s.contains("Reply"))
        .unwrap_or("No timestamp")
        .to_string();

    let content_regex: Regex = Regex::new(r#"white-space:pre-wrap;width:100%">(.*?)</div>"#).ok()?;
    let content: &str = content_regex.captures(raw_content)?.get(1)?.as_str().trim();

    let clean_content: String = clean_message_content(content);

    Some(Message {
        message_id,
        sender,
        timestamp,
        content: clean_content,
    })
}

fn clean_message_content(content: &str) -> String {
    let tag_regex: Regex = Regex::new(r"<[^>]+>").unwrap();
    let without_tags: Cow<str> = tag_regex.replace_all(content, "");

    let decoded: String = decode_html_entities(&without_tags).to_string();

    let reply_regex: Regex = Regex::new(r"^\s*\d+\s+Reply\s*").unwrap();
    let without_reply: Cow<str> = reply_regex.replace(&decoded, "");

    without_reply.trim().to_string()
}