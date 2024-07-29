use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::error::Error;
use base64::{prelude::BASE64_STANDARD, Engine as _};
use crate::models::Attachment;

/// Reads the content of an .mbox file into a String.
///
/// # Arguments
///
/// * `path` - A path-like object representing the location of the .mbox file.
///
/// # Returns
///
/// A Result containing the file content as a String if successful, or a boxed error if reading fails.
///
/// # Errors
///
/// This function will return an error if the file cannot be opened or read.
pub fn read_mbox_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut file: File = File::open(path)?;
    let mut content: String = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Writes an attachment to a file in the specified folder.
///
/// # Arguments
///
/// * `attachment` - A reference to an Attachment struct containing the attachment data.
/// * `folder` - A string slice specifying the folder where the attachment should be saved.
///
/// # Returns
///
/// A Result indicating success (Ok(())) or failure (Err) in writing the attachment.
///
/// # Errors
///
/// This function will return an error if creating the folder,
/// decoding the content, or writing the file fails.
pub fn write_attachment_to_file(attachment: &Attachment, folder: &str) -> Result<(), Box<dyn Error>> {
    let folder_path: &Path = Path::new(folder);
    if !folder_path.exists() {
        fs::create_dir_all(folder_path)?;
    }

    let file_path: PathBuf = folder_path.join(&attachment.filename);
    let decoded_content: Vec<u8> = BASE64_STANDARD.decode(&attachment.content)?;

    fs::write(file_path, decoded_content)?;

    Ok(())
}