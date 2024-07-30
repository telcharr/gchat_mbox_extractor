# Google Chat MBOX Extractor

## What's this?

If you're an IT pro or Google Workspace admin,
you've probably encountered the joy (read: pain) of dealing with Google Chat .mbox exports.
I looked for a tool to parse these .mbox exports, but found nothing.
So, I made my own.
This app is designed to help you dig through and analyze Google Chat MBOX archives without losing your sanity.

## What does it do?

This app takes your Google Chat MBOX file and:
1. Extracts all the messages
2. Saves them in a nice, readable CSV format
3. Optionally extracts any attachments and saves them

All wrapped up in a (somewhat) pretty GUI package.

## How to use it?

1. Launch the application
2. Click "Select MBOX File" and choose your MBOX file
3. Click "Select Output Folder" and pick your desired output location
4. Decide if you want to extract attachments (check the box if so)
5. Click "Process MBOX" to start the extraction
6. Once complete, you'll find your extracted data in the output folder

## Building from source

If you'd like to build the application yourself:

1. Ensure you have Rust installed (if not, visit [rustup.rs](https://rustup.rs))
2. Clone this repository
3. Run `cargo build --release`
4. Find the executable in `target/release`
5. Run the executable to start the application

## A note on the UI

UI design isn't my strong suit. If you open this up and your eyes hurt, I apologize in advance. If there are any UI wizards out there who want to take a crack at making this look less like it was designed by backend engineer (me), please feel free to submit a PR.

## Contributing

Found a bug? Have an idea for a feature? Want to tell me my code looks like spaghetti? Feel free to open an issue or submit a PR.