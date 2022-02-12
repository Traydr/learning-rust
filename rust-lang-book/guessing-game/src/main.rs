// Imports the standard io library
use std::io;

// The main entry point to the program
fn main() {
    // This macro prints to the screen
    println!("Guess a number! (input below)");

    // Creates a variable changeable variables of type string
    let mut guess = String::new();

    // Reads from input, else throws error message
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Outputs user input
    println!("You guessed: {}", guess);
}
