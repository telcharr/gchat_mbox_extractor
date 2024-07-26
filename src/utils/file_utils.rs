use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::error::Error;
use base64::{prelude::BASE64_STANDARD, Engine as _};
use crate::models::Attachment;

pub fn read_mbox_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut file: File = File::open(path)?;
    let mut content: String = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_attachment_to_file(attachment: &Attachment, folder: &str) -> Result<(), Box<dyn Error>> {
    let folder_path: &Path = Path::new(folder);
    if !folder_path.exists() {
        fs::create_dir_all(folder_path)?;
    }

    let file_path: PathBuf = folder_path.join(&attachment.filename);

    // Decode the base64 content
    let decoded_content: Vec<u8> = BASE64_STANDARD.decode(&attachment.content)?;

    // Write the decoded content to the file
    fs::write(file_path, decoded_content)?;

    Ok(())
}