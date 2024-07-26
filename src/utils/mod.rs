mod file_utils;

pub use file_utils::read_mbox_file;

use quoted_printable::ParseMode;
use std::error::Error;

pub fn decode_quoted_printable(input: &str) -> Result<String, Box<dyn Error>> {
    let decoded = quoted_printable::decode(input.as_bytes(), ParseMode::Robust)?;
    Ok(String::from_utf8_lossy(&decoded).to_string())
}