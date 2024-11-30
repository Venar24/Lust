use std::env;
use std::fs;
use std::process;
use std::io;
use std::time::UNIX_EPOCH;
use chrono::{DateTime, Local};

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let show_hidden = args.contains(&"-c".to_string());
    let show_help = args.contains(&"-help".to_string());

if show_help{
    println!("Here are the Lust arguments.
    -help to see all the possible arguments.
    -c    to see hidden files.");
    process::exit(0); // Exit with a success code
}

    // Read and list the file names in the current directory
    for entry in fs::read_dir(std::env::current_dir()?)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip hidden files unless the -c argument is passed
        if show_hidden || !file_name_str.starts_with('.') {
            // Get file metadata
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    // Convert to a human-readable timestamp
                    let duration_since_epoch = modified.duration_since(UNIX_EPOCH).unwrap_or_default();
                    let datetime = DateTime::<Local>::from(UNIX_EPOCH + duration_since_epoch);
                    println!("{} - {}", datetime.format("%Y-%m-%d %H:%M"), file_name_str);
                } else {
                    println!("N/A - {}", file_name_str);
                }
            }
        }
    }

    Ok(())
}
