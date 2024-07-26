mod models;
mod parsers;
mod utils;

use models::MboxEntry;
use parsers::split_mbox_entries;
use utils::read_mbox_file;
use std::error::Error;
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "Customer_Succes_Advisory_Chat_History_0.mbox";
    let mbox_content: String = read_mbox_file(path)?;
    println!("File content length: {}", mbox_content.len());

    let mbox_entries: Vec<MboxEntry> = split_mbox_entries(&mbox_content)?;
    println!("Number of MBOX entries: {}", mbox_entries.len());

    // Process entries in parallel and collect results
    let mut entry_outputs: Vec<(usize, String)> = mbox_entries.par_iter().enumerate()
        .map(|(i, entry)| {
            let mut output: String = String::new();

            output.push_str(&format!("Entry {}: Header length: {}, HTML body length: {}, Attachments: {}, Messages: {}\n\n",
                                     i + 1, entry.headers.len(), entry.html_body.len(), entry.attachments.len(), entry.messages.len()));

            output.push_str(&format!("Headers:\n{}\n\n", &entry.headers));

            for (j, message) in entry.messages.iter().enumerate().take(3) {
                output.push_str(&format!("Message {}:\n", j + 1));
                output.push_str(&format!("  Message ID: {}\n", message.message_id));
                output.push_str(&format!("  Sender: {}\n", message.sender));
                output.push_str(&format!("  Timestamp: {}\n", message.timestamp));
                output.push_str(&format!("  Content: {}\n\n", message.content));
            }
            if entry.messages.len() > 3 {
                output.push_str(&format!("... ({} more messages)\n", entry.messages.len() - 3));
            }

            if !entry.attachments.is_empty() {
                output.push_str("\nAttachments:\n");
                for (j, attachment) in entry.attachments.iter().enumerate() {
                    output.push_str(&format!("  {}: {} ({})\n", j + 1, attachment.filename, attachment.content_type));
                }
            }

            output.push_str("\n-----------------------\n\n");

            (i, output)
        })
        .collect();

    // Sort the outputs by their original index
    entry_outputs.sort_by_key(|&(i, _)| i);

    // Print the sorted outputs
    for (_, output) in entry_outputs {
        print!("{}", output);
    }

    Ok(())
}