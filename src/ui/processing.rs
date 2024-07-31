use std::path::PathBuf;
use std::error::Error;
use std::sync::mpsc::{Sender, SendError};

use crate::models::{MboxEntry, Message};
use crate::parsers::split_mbox_entries;
use crate::utils::{read_mbox_file, write_messages_to_csv, write_attachment_to_file};

const FILE_READ_WEIGHT: f32 = 0.1;
const PARSING_WEIGHT: f32 = 0.2;
const PROCESSING_WEIGHT: f32 = 0.6;
const WRITING_WEIGHT: f32 = 0.1;

/// Called to execute the `do_process_mbox` function, match the result and transmit
pub fn process_mbox(
    mbox_path: &PathBuf,
    output_path: &PathBuf,
    export_attachments: bool,
    progress_tx: Sender<f32>,
    result_tx: Sender<String>,
) {
    let result: Result<(), Box<dyn Error>> = do_process_mbox(mbox_path, output_path, export_attachments, &progress_tx);
    let message: String = match result {
        Ok(()) => "Processing completed successfully.".to_string(),
        Err(e) => format!("Error: {}", e),
    };
    result_tx.send(message).unwrap_or_else(|e: SendError<String>| eprintln!("Failed to send result: {}", e));
}

/// Handles the core logic of parsing the MBOX file, extracting messages and attachments, and writing the results to the specified output location
fn do_process_mbox(
    mbox_path: &PathBuf,
    output_path: &PathBuf,
    export_attachments: bool,
    progress_tx: &Sender<f32>,
) -> Result<(), Box<dyn Error>> {
    // Step 1: Read MBOX file
    send_progress(progress_tx, 0.0);
    let mbox_content: String = read_mbox_file(mbox_path)?;
    send_progress(progress_tx, FILE_READ_WEIGHT);

    // Step 2: Parse MBOX entries
    let mbox_entries: Vec<MboxEntry> = split_mbox_entries(&mbox_content)?;
    send_progress(progress_tx, FILE_READ_WEIGHT + PARSING_WEIGHT);

    // Step 3: Process entries
    let mut all_messages: Vec<Message> = Vec::new();
    let total_entries: usize = mbox_entries.len();

    for (index, entry) in mbox_entries.iter().enumerate() {
        all_messages.extend(entry.messages.clone());

        if export_attachments {
            let attachments_folder: PathBuf = output_path.join("attachments");
            for attachment in &entry.attachments {
                write_attachment_to_file(attachment, attachments_folder.to_str().ok_or("Invalid path")?)?;
            }
        }

        let progress: f32 = FILE_READ_WEIGHT + PARSING_WEIGHT + PROCESSING_WEIGHT * (index + 1) as f32 / total_entries as f32;
        send_progress(progress_tx, progress);
    }

    // Step 4: Write CSV
    let csv_path: PathBuf = output_path.join("messages.csv");
    send_progress(progress_tx, FILE_READ_WEIGHT + PARSING_WEIGHT + PROCESSING_WEIGHT);

    write_messages_to_csv(&all_messages, csv_path.to_str().ok_or("Invalid path")?, |csv_progress: f32| {
        let overall_progress: f32 = FILE_READ_WEIGHT + PARSING_WEIGHT + PROCESSING_WEIGHT + WRITING_WEIGHT * csv_progress;
        send_progress(progress_tx, overall_progress);
        Ok(())
    })?;

    Ok(())
}

/// Helper function to send progress updates
fn send_progress(progress_tx: &Sender<f32>, progress: f32) {
    progress_tx.send(progress)
        .unwrap_or_else(|e: SendError<f32>| eprintln!("Failed to send progress: {}", e));
}