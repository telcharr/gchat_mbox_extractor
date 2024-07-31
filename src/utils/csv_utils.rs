use std::error::Error;
use std::fs::File;
use std::io::Write;
use crate::models::Message;

/// Writes a collection of Message structs to a CSV file with progress updates.
///
/// # Arguments
///
/// * `messages` - A slice of Message structs to be written to the CSV file.
/// * `filename` - A string slice specifying the name of the output CSV file.
/// * `progress_callback` - A closure that will be called with progress updates.
///
/// # Returns
///
/// A Result indicating success (Ok(())) or failure (Err) in writing the CSV file.
///
/// # Errors
///
/// This function will return an error if creating the file or writing to it fails.
pub fn write_messages_to_csv<F>(
    messages: &[Message],
    filename: &str,
    mut progress_callback: F
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(f32) -> Result<(), Box<dyn Error>>
{
    let mut file: File = File::create(filename)?;

    // Write CSV header
    writeln!(file, "message_id,sender,timestamp,content")?;

    let total_messages: usize = messages.len();

    // Write each message as a CSV row
    for (index, message) in messages.iter().enumerate() {
        let content: String = message.content.replace("\"", "\"\"");
        writeln!(
            file,
            "\"{}\",\"{}\",\"{}\",\"{}\"",
            message.message_id,
            message.sender,
            message.timestamp,
            content
        )?;

        // Call the progress callback every 100 messages or on the last message
        if index % 100 == 0 || index == total_messages - 1 {
            let progress = (index + 1) as f32 / total_messages as f32;
            progress_callback(progress)?;
        }
    }

    Ok(())
}