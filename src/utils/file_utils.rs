use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

pub fn read_mbox_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}