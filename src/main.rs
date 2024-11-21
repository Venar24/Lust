//fn main() {
//
//    let game_class = std::env::args().nth(1).expect("no argument given");
//
//    println!("toto est un {}.", game_class);
//}

use std::io::{self, Write};

fn main() {
    // Ask the user for input
    print!("Enter toto's Class: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

    let mut input = String::new();
    
    // Read the user input
    io::stdin().read_line(&mut input).unwrap();

    // Print the user input
    println!("Toto's class is: {}", input.trim()); // Use trim() to remove any trailing newline
}

