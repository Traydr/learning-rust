// Imports the standard io library
use std::io;
// Imports the rng crate
use rand::Rng;

// The main entry point to the program
fn main() {
    // This macro prints to the screen
    println!("Guess a number!");

    // Creates an immutable var with a range from 1 to 101
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret number: {}", secret_number);
    println!("Input below");

    // Creates a variable changeable variables of type string
    let mut guess = String::new();

    // Reads from input, else throws error message
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Outputs user input
    println!("You guessed: {}", guess);
}
