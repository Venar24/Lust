use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Get the current directory
    let current_dir = std::env::current_dir()?;
    //println!("Current directory: {}", current_dir.display());

    // Read and list the contents of the current directory
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }

    Ok(())
}
