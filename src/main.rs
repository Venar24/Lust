use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let show_hidden = args.contains(&"-c".to_string());

    // Read and list the contents of the current directory
    for entry in fs::read_dir(std::env::current_dir()?)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip hidden files (file names starting with '.')
        if show_hidden || !file_name_str.starts_with('.') {
            println!("{}", file_name_str);
        }
    }

    Ok(())
}