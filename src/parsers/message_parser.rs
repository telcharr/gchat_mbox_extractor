use std::borrow::Cow;
use crate::models::{RawMessage, Message};
use regex::{Captures, Match, Regex};
use html_escape::decode_html_entities;
use rayon::prelude::*;
use chrono::{DateTime, FixedOffset, TimeZone, NaiveDate, NaiveTime, NaiveDateTime};

/// Splits the HTML content into individual raw messages.
///
/// # Arguments
///
/// * `html` - A string slice containing the entire HTML content of the .mbox file.
///
/// # Returns
///
/// A vector of `RawMessage` structs, each containing the content of a single message.
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

/// Parses a raw message content into a structured `Message` object.
///
/// # Arguments
///
/// * `raw_content` - A string slice containing the raw HTML content of a single message.
///
/// # Returns
///
/// An `Option<Message>` containing the parsed message if successful, or `None` if parsing fails.
pub fn parse_message(raw_content: &str) -> Option<Message> {
    let id_regex: Regex = Regex::new(r#"<div data-id="([^"]+)""#).ok()?;
    let message_id: String = id_regex.captures(raw_content)?.get(1)?.as_str().to_string();

    let sender_regex: Regex = Regex::new(r#"<span style="font-weight:700">(.*?)</span>"#).ok()?;
    let sender: String = sender_regex.captures(raw_content)?.get(1)?.as_str().trim().to_string();

    let timestamp_regex: Regex = Regex::new(r#"(?s)<div><span style="font-weight:700">.*?</span>(.*?)</div>"#).ok()?;
    let raw_timestamp: String = timestamp_regex.captures(raw_content)
        .and_then(|cap: Captures| cap.get(1))
        .map(|m: Match| m.as_str().trim())
        .unwrap_or("No timestamp")
        .to_string();

    let timestamp: String = parse_and_format_timestamp(&raw_timestamp);

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

/// Parse and format a raw timestamp string into RFC 3339 format.
///
/// # Arguments
///
/// * `raw_timestamp` - A string slice containing the raw timestamp from the message.
///
/// # Returns
///
/// A `String` containing the parsed and formatted timestamp in RFC 3339 format.
fn parse_and_format_timestamp(raw_timestamp: &str) -> String {
    // Replace non-breaking space with regular space
    let cleaned_timestamp: String = raw_timestamp.replace('\u{202F}', " ");

    // Define a regex pattern to match the timestamp components
    let re: Regex = Regex::new(r"(\w+ \d{1,2}, \d{4}) at (\d{1,2}):(\d{2}):(\d{2}) (AM|PM) GMT(-?\d+)").unwrap();

    if let Some(captures) = re.captures(&cleaned_timestamp) {
        let date_str: &str = captures.get(1).unwrap().as_str();
        let hour: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let minute: u32 = captures.get(3).unwrap().as_str().parse().unwrap();
        let second: u32 = captures.get(4).unwrap().as_str().parse().unwrap();
        let am_pm: &str = captures.get(5).unwrap().as_str();
        let offset: i32 = captures.get(6).unwrap().as_str().parse().unwrap();

        // Adjust hour for PM
        let hour: u32 = if am_pm == "PM" && hour != 12 { hour + 12 } else if am_pm == "AM" && hour == 12 { 0 } else { hour };

        if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, "%B %d, %Y") {
            let naive_time: NaiveTime = NaiveTime::from_hms_opt(hour, minute, second).unwrap_or_default();
            let naive_dt: NaiveDateTime = naive_date.and_time(naive_time);

            let offset: FixedOffset = FixedOffset::east_opt(offset * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap());
            let datetime: DateTime<FixedOffset> = offset.from_local_datetime(&naive_dt).single().unwrap_or(offset.from_utc_datetime(&naive_dt));
            return datetime.to_rfc3339();
        }
    }

    eprintln!("Failed to parse timestamp '{}'", cleaned_timestamp);
    raw_timestamp.to_string()
}

/// Cleans the message content by removing HTML tags, decoding entities, and trimming unnecessary text.
///
/// # Arguments
///
/// * `content` - A string slice containing the raw message content.
///
/// # Returns
///
/// A `String` containing the cleaned message content.
fn clean_message_content(content: &str) -> String {
    let tag_regex: Regex = Regex::new(r"<[^>]+>").unwrap();
    let without_tags: Cow<str> = tag_regex.replace_all(content, "");

    let decoded: String = decode_html_entities(&without_tags).to_string();

    let reply_regex: Regex = Regex::new(r"^\s*\d+\s+Reply\s*").unwrap();
    let without_reply: Cow<str> = reply_regex.replace(&decoded, "");

    without_reply.trim().to_string()
}