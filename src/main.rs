use std::env;
use std::fs;
use std::process;
use std::io;
use std::time::UNIX_EPOCH;
use chrono::{DateTime, Local};

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();    
    if args.contains(&"-i".to_string()) {
        show_hidden()?; // Propagate errors from `show_hidden`
    }
   else if args.contains(&"-h".to_string()) || args.contains(&"-help".to_string()) {
        show_help(); // This doesn't need `Result`, as it exits the program
    }
    else {
        show_files()?;
    }

    Ok(())
}

fn show_help() {
    println!(
        "Here are the Lust arguments:
    -help or -h to see all the possible arguments.
    -i    to see hidden files."
    );
    process::exit(0); // Exit with a success code
}

fn show_hidden() -> io::Result<()> {
    // Read and list the file names in the current directory
    for entry in fs::read_dir(env::current_dir()?)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        //if file_name_str.starts_with('.') {
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
        //}
    }
    Ok(())
}

    fn show_files() -> io::Result<()> {
        // Read and list the file names in the current directory
        for entry in fs::read_dir(env::current_dir()?)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
    
            if !file_name_str.starts_with('.') {
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
            //}
        }
    }

    Ok(())
}
