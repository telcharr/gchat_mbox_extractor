use std::path::PathBuf;
use std::error::Error;
use std::sync::mpsc::Sender;

use crate::models::{MboxEntry, Message};
use crate::parsers::split_mbox_entries;
use crate::utils::{read_mbox_file, write_messages_to_csv, write_attachment_to_file};

/// Called to execute the `do_process_mbox` function, match the result and transmit
pub fn process_mbox(
    mbox_path: &PathBuf,
    output_path: &PathBuf,
    export_attachments: bool,
    progress_tx: Sender<f32>,
    result_tx: Sender<String>,
) {
    let result: Result<(), Box<dyn Error>> = do_process_mbox(mbox_path, output_path, export_attachments, progress_tx);
    match result {
        Ok(()) => result_tx.send("Processing completed successfully.".to_string()).unwrap(),
        Err(e) => result_tx.send(format!("Error: {}", e)).unwrap(),
    }
}

/// Handles the core logic of parsing the MBOX file, extracting messages and attachments, and writing the results to the specified output location
fn do_process_mbox(
    mbox_path: &PathBuf,
    output_path: &PathBuf,
    export_attachments: bool,
    progress_tx: Sender<f32>,
) -> Result<(), Box<dyn Error>> {
    let mbox_content: String = read_mbox_file(mbox_path)?;
    let mbox_entries: Vec<MboxEntry> = split_mbox_entries(&mbox_content)?;

    let mut all_messages: Vec<Message> = Vec::new();
    let total_entries: usize = mbox_entries.len();

    for (index, entry) in mbox_entries.iter().enumerate() {
        all_messages.extend(entry.messages.clone());

        if export_attachments {
            let attachments_folder: PathBuf = output_path.join("attachments");
            for attachment in &entry.attachments {
                write_attachment_to_file(attachment, attachments_folder.to_str().unwrap())?;
            }
        }

        progress_tx.send((index + 1) as f32 / total_entries as f32).unwrap();
    }

    let csv_path: PathBuf = output_path.join("messages.csv");
    write_messages_to_csv(&all_messages, csv_path.to_str().unwrap())?;

    progress_tx.send(1.0).unwrap();

    Ok(())
}