mod models;
mod parsers;
mod utils;
mod ui;

use ui::run_ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_ui()?;
    Ok(())
}