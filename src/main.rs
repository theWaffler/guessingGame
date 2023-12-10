use std::io::{self, Write};

fn main() {
    println!("Guess the fucking number!"); // adds new line
    print!("Please input your guess: "); // doesnt add new line

    io::stdout().flush().expect("Failed to flush stdout"); // Flush the buffer

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) //mutable. default is immutable
        .expect("Failed to read line");
    println!("You Guess: {}", guess.trim());
}
