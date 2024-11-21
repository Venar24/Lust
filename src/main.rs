use std::env;
use std::fs;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::NaiveDateTime;

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let show_hidden = args.contains(&"-c".to_string());

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
                    let duration = modified.duration_since(UNIX_EPOCH).unwrap_or_default();
                    let timestamp = duration.as_secs();
                    let datetime = NaiveDateTime::from_timestamp(timestamp as i64, 0);
                    println!("{} - {}", datetime, file_name_str);
                } else {
                    println!("N/A - {}", file_name_str);
                }
            }
        }
    }

    Ok(())
}
