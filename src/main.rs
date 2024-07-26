mod models;
mod parsers;
mod utils;

use models::MboxEntry;
use parsers::split_mbox_entries;
use utils::read_mbox_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "path_to_file.mbox";
    let mbox_content: String = read_mbox_file(path)?;
    println!("File content length: {}", mbox_content.len());

    let mbox_entries: Vec<MboxEntry> = split_mbox_entries(&mbox_content)?;
    println!("Number of MBOX entries: {}", mbox_entries.len());

    for (i, entry) in mbox_entries.iter().enumerate() {
        println!("Entry {}: Header length: {}, HTML body length: {}, Attachments: {}, Messages: {}",
                 i + 1, entry.headers.len(), entry.html_body.len(), entry.attachments.len(), entry.messages.len());

        println!("Headers:\n{}\n", &entry.headers);

        for (j, message) in entry.messages.iter().enumerate().take(3) {
            println!("Message {}:", j + 1);
            println!("  Message ID: {}", message.message_id);
            println!("  Sender: {}", message.sender);
            println!("  Timestamp: {}", message.timestamp);
            println!("  Content: {}\n", message.content);
        }
        if entry.messages.len() > 3 {
            println!("... ({} more messages)", entry.messages.len() - 3);
        }

        if !entry.attachments.is_empty() {
            println!("\nAttachments:");
            for (j, attachment) in entry.attachments.iter().enumerate() {
                println!("  {}: {} ({})", j + 1, attachment.filename, attachment.content_type);
            }
        }

        println!("\n-----------------------\n");
    }

    Ok(())
}