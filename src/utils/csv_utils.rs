use std::error::Error;
use std::fs::File;
use std::io::Write;
use crate::models::Message;

pub fn write_messages_to_csv(messages: &[Message], filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file: File = File::create(filename)?;

    // Write CSV header
    writeln!(file, "message_id,sender,timestamp,content")?;

    // Write each message as a CSV row
    for message in messages {
        let content: String = message.content.replace("\"", "\"\""); // Escape double quotes
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