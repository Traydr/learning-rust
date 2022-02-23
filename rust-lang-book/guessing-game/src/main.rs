// Imports the standard io library
use std::io;
// Imports the rng crate
use rand::Rng;
// Imports comparisons
use std::cmp::Ordering;

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

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // Outputs user input
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}
