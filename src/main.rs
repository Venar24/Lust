use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Get the current directory
    let current_dir = std::env::current_dir()?;
    //println!("Current directory: {}", current_dir.display());

    // Read and list the contents of the current directory
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip hidden files (file names starting with '.')
        if !file_name_str.starts_with('.') {
            println!("{}", file_name_str);
        }
    }

    Ok(())
}
