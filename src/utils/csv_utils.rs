use std::error::Error;
use std::fs::File;
use std::io::Write;
use crate::models::Message;

/// Writes a collection of Message structs to a CSV file.
///
/// # Arguments
///
/// * `messages` - A slice of Message structs to be written to the CSV file.
/// * `filename` - A string slice specifying the name of the output CSV file.
///
/// # Returns
///
/// A Result indicating success (Ok(())) or failure (Err) in writing the CSV file.
///
/// # Errors
///
/// This function will return an error if creating the file or writing to it fails.
pub fn write_messages_to_csv(messages: &[Message], filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file: File = File::create(filename)?;

    // Write CSV header
    writeln!(file, "message_id,sender,timestamp,content")?;

    // Write each message as a CSV row
    for message in messages {
        let content: String = message.content.replace("\"", "\"\"");
        writeln!(
            file,
            "\"{}\",\"{}\",\"{}\",\"{}\"",
            message.message_id,
            message.sender,
            message.timestamp,
            content
        )?;
    }

    Ok(())
}