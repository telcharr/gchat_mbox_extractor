mod file_utils;
mod csv_utils;

pub use file_utils::{read_mbox_file, write_attachment_to_file};
pub use csv_utils::write_messages_to_csv;

use quoted_printable::ParseMode;
use std::error::Error;

pub fn decode_quoted_printable(input: &str) -> Result<String, Box<dyn Error>> {
    let decoded: Vec<u8> = quoted_printable::decode(input.as_bytes(), ParseMode::Robust)?;
    Ok(String::from_utf8_lossy(&decoded).to_string())
}